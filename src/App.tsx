import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Settings, Mic, Volume2, Radio, Smartphone, LayoutGrid, Save } from "lucide-react";
import { RoutingMatrix } from "./components/RoutingMatrix";
import { VuMeter } from "./components/VuMeter";
import { DeviceList } from "./components/DeviceList";
import { FxChain } from "./components/FxChain";
import { Presets } from "./components/Presets";

interface Device {
  id: string;
  name: string;
  deviceType: string;
  sampleRate: number;
  channels: number;
}

interface BackendDevice {
  id: string;
  name: string;
  deviceType?: string;
  device_type?: string;
  sampleRate?: number;
  sample_rate?: number;
  channels: number;
}

interface Route {
  inputId: string;
  outputId: string;
  volume: number;
  muted: boolean;
}

interface BackendRoute {
  inputId?: string;
  outputId?: string;
  input_id?: string;
  output_id?: string;
  volume: number;
  muted: boolean;
}

interface DspSettings {
  gain: number;
  noise_gate_enabled: boolean;
  noise_gate_threshold: number;
  eq_bands: number[];
  compressor_enabled: boolean;
  compressor_threshold: number;
  compressor_ratio: number;
  compressor_attack: number;
  compressor_release: number;
}

function App() {
  const [activeTab, setActiveTab] = useState("dashboard");
  const [inputDevices, setInputDevices] = useState<Device[]>([]);
  const [outputDevices, setOutputDevices] = useState<Device[]>([]);
  const [routes, setRoutes] = useState<Route[]>([]);
  const [selectedInput, setSelectedInput] = useState<string | null>(null);
  const [selectedOutput, setSelectedOutput] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [newDeviceName, setNewDeviceName] = useState("");
  const [presetNames, setPresetNames] = useState<string[]>([]);
  const [dspSettings, setDspSettings] = useState<Record<string, DspSettings>>({});
  const [toast, setToast] = useState<{ type: "success" | "error"; message: string } | null>(null);

  const notify = useCallback((type: "success" | "error", message: string) => {
    setToast({ type, message });
    window.setTimeout(() => setToast(null), 2800);
  }, []);

  const loadDevices = useCallback(async () => {
    try {
      const devices = await invoke<BackendDevice[]>("get_audio_devices");
      const mapped: Device[] = devices.map((d) => ({
        id: d.id,
        name: d.name,
        deviceType: d.deviceType ?? d.device_type ?? "Unknown",
        sampleRate: d.sampleRate ?? d.sample_rate ?? 48000,
        channels: d.channels,
      }));
      const inputs = mapped.filter(
        (d) =>
          d.deviceType.includes("Microphone") ||
          d.deviceType.includes("Network") ||
          d.id === "loopback_default"
      );
      const outputs = mapped.filter((d) => d.deviceType.includes("SystemAudio") || d.deviceType.includes("Virtual"));
      setInputDevices(inputs);
      setOutputDevices(outputs);
    } catch (error) {
      console.error("Failed to load devices:", error);
      setInputDevices([
        { id: "mic_1", name: "Default Microphone", deviceType: "Microphone", sampleRate: 48000, channels: 2 },
        { id: "sys_1", name: "System Audio", deviceType: "SystemAudio", sampleRate: 48000, channels: 2 },
      ]);
      setOutputDevices([
        { id: "vac_1", name: "VAC-1 (Virtual)", deviceType: "Virtual", sampleRate: 48000, channels: 2 },
        { id: "vac_2", name: "VAC-2 (Virtual)", deviceType: "Virtual", sampleRate: 48000, channels: 2 },
      ]);
    } finally {
      setIsLoading(false);
    }
  }, []);

  const loadRoutes = useCallback(async () => {
    try {
      const backendRoutes = await invoke<BackendRoute[]>("get_routes");
      setRoutes(
        backendRoutes.map((r) => ({
          inputId: r.inputId ?? r.input_id ?? "",
          outputId: r.outputId ?? r.output_id ?? "",
          volume: r.volume,
          muted: r.muted,
        }))
      );
    } catch (error) {
      console.error("Failed to load routes:", error);
      notify("error", "Failed to load routes");
    }
  }, [notify]);

  const loadPresets = useCallback(async () => {
    try {
      const names = await invoke<string[]>("get_presets");
      setPresetNames(names);
    } catch (error) {
      console.error("Failed to load presets:", error);
      notify("error", "Failed to load presets");
    }
  }, [notify]);

  const loadDspSettings = useCallback(async (deviceId: string) => {
    try {
      const settings = await invoke<DspSettings>("get_device_dsp", { device_id: deviceId });
      setDspSettings((prev) => ({ ...prev, [deviceId]: settings }));
    } catch {
      // No DSP config yet — that's fine
    }
  }, []);

  useEffect(() => {
    loadDevices();
    loadRoutes();
    loadPresets();
  }, [loadDevices, loadRoutes, loadPresets]);

  useEffect(() => {
    if (routes.length > 0) {
      loadDspSettings(routes[0].inputId);
    }
  }, [routes, loadDspSettings]);

  const handleRouteToggle = async (inputId: string, outputId: string) => {
    const existingRoute = routes.find((r) => r.inputId === inputId && r.outputId === outputId);
    if (existingRoute) {
      try {
        await invoke("remove_route", { input_id: inputId, output_id: outputId });
        setRoutes(routes.filter((r) => r !== existingRoute));
      } catch (error) {
        console.error("Failed to remove route:", error);
        notify("error", "Failed to remove route");
      }
    } else {
      try {
        await invoke("set_route", { input_id: inputId, output_id: outputId, volume: 100, muted: false });
        setRoutes([...routes, { inputId, outputId, volume: 100, muted: false }]);
      } catch (error) {
        console.error("Failed to set route:", error);
        notify("error", "Failed to set route");
      }
    }
  };

  const handleVolumeChange = async (inputId: string, outputId: string, volume: number) => {
    setRoutes(
      routes.map((r) =>
        r.inputId === inputId && r.outputId === outputId ? { ...r, volume } : r
      )
    );
    try {
      await invoke("set_volume", { input_id: inputId, output_id: outputId, volume });
    } catch (error) {
      console.error("Failed to set volume:", error);
      notify("error", "Failed to update volume");
    }
  };

  const handleMuteToggle = async (inputId: string, outputId: string) => {
    const route = routes.find((r) => r.inputId === inputId && r.outputId === outputId);
    if (!route) return;
    const newMuted = !route.muted;
    setRoutes(
      routes.map((r) =>
        r.inputId === inputId && r.outputId === outputId ? { ...r, muted: newMuted } : r
      )
    );
    try {
      await invoke("set_mute", { input_id: inputId, output_id: outputId, muted: newMuted });
    } catch (error) {
      console.error("Failed to set mute:", error);
      notify("error", "Failed to update mute");
    }
  };

  const handleCreateDevice = async () => {
    if (!newDeviceName.trim()) return;
    try {
      await invoke("create_virtual_device", { name: newDeviceName, channels: 2 });
      setNewDeviceName("");
      await loadDevices();
    } catch (error) {
      console.error("Failed to create device:", error);
      notify("error", "Failed to create virtual device");
    }
  };

  const handleSavePreset = async (name: string) => {
    try {
      await invoke("save_preset", { name });
      await loadPresets();
      notify("success", `Preset saved: ${name}`);
    } catch (error) {
      console.error("Failed to save preset:", error);
      notify("error", "Failed to save preset");
    }
  };

  const handleLoadPreset = async (name: string) => {
    try {
      const preset = await invoke<{ name: string; routes: BackendRoute[] }>("load_preset", { name });
      setRoutes(
        preset.routes.map((r) => ({
          inputId: r.inputId ?? r.input_id ?? "",
          outputId: r.outputId ?? r.output_id ?? "",
          volume: r.volume,
          muted: r.muted,
        }))
      );
      await loadDevices();
      notify("success", `Preset loaded: ${name}`);
    } catch (error) {
      console.error("Failed to load preset:", error);
      notify("error", "Failed to load preset");
    }
  };

  const handleDeletePreset = async (name: string) => {
    try {
      await invoke("delete_preset", { name });
      await loadPresets();
      notify("success", `Preset deleted: ${name}`);
    } catch (error) {
      console.error("Failed to delete preset:", error);
      notify("error", "Failed to delete preset");
    }
  };

  const handleExportPreset = async (name: string) => {
    try {
      const presetJson = await invoke<string>("export_preset", { name });
      const blob = new Blob([presetJson], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `${name}.json`;
      a.click();
      URL.revokeObjectURL(url);
      notify("success", `Preset exported: ${name}`);
    } catch (error) {
      console.error("Failed to export preset:", error);
      notify("error", "Failed to export preset");
    }
  };

  const handleImportPreset = async (file: File) => {
    try {
      const presetJson = await file.text();
      const name = await invoke<string>("import_preset", { presetJson });
      await loadPresets();
      notify("success", `Preset imported: ${name}`);
    } catch (error) {
      console.error("Failed to import preset:", error);
      notify("error", "Failed to import preset");
    }
  };

  const handleDspChange = async (deviceId: string, settings: DspSettings) => {
    setDspSettings((prev) => ({ ...prev, [deviceId]: settings }));
    try {
      await invoke("set_device_dsp", { device_id: deviceId, settings });
    } catch (error) {
      console.error("Failed to set DSP:", error);
      notify("error", "Failed to apply DSP settings");
    }
  };

  const handleAddRoute = () => {
    if (inputDevices.length > 0 && outputDevices.length > 0) {
      const inputId = selectedInput || inputDevices[0].id;
      const outputId = selectedOutput || outputDevices[0].id;
      handleRouteToggle(inputId, outputId);
    }
  };

  const loopbackActive = routes.some((r) => r.inputId === "loopback_default");

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      <header className="bg-gray-800 border-b border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Radio className="w-8 h-8 text-primary-500" />
            <h1 className="text-2xl font-bold">Virtual Audio Cable</h1>
            {loopbackActive && (
              <span className="px-2 py-1 text-xs rounded-md bg-emerald-700/70 border border-emerald-400/50 text-emerald-100">
                Loopback Active
              </span>
            )}
          </div>
          <nav className="flex gap-2">
            <button
              onClick={() => setActiveTab("dashboard")}
              className={`px-4 py-2 rounded-lg flex items-center gap-2 transition-colors ${
                activeTab === "dashboard"
                  ? "bg-primary-600 text-white"
                  : "bg-gray-700 hover:bg-gray-600"
              }`}
            >
              <LayoutGrid className="w-4 h-4" />
              Dashboard
            </button>
            <button
              onClick={() => setActiveTab("devices")}
              className={`px-4 py-2 rounded-lg flex items-center gap-2 transition-colors ${
                activeTab === "devices"
                  ? "bg-primary-600 text-white"
                  : "bg-gray-700 hover:bg-gray-600"
              }`}
            >
              <Mic className="w-4 h-4" />
              Devices
            </button>
            <button
              onClick={() => setActiveTab("fx")}
              className={`px-4 py-2 rounded-lg flex items-center gap-2 transition-colors ${
                activeTab === "fx"
                  ? "bg-primary-600 text-white"
                  : "bg-gray-700 hover:bg-gray-600"
              }`}
            >
              <Volume2 className="w-4 h-4" />
              FX Chain
            </button>
            <button
              onClick={() => setActiveTab("presets")}
              className={`px-4 py-2 rounded-lg flex items-center gap-2 transition-colors ${
                activeTab === "presets"
                  ? "bg-primary-600 text-white"
                  : "bg-gray-700 hover:bg-gray-600"
              }`}
            >
              <Save className="w-4 h-4" />
              Presets
            </button>
            <button
              onClick={() => setActiveTab("mobile")}
              className={`px-4 py-2 rounded-lg flex items-center gap-2 transition-colors ${
                activeTab === "mobile"
                  ? "bg-primary-600 text-white"
                  : "bg-gray-700 hover:bg-gray-600"
              }`}
            >
              <Smartphone className="w-4 h-4" />
              Mobile
            </button>
            <button
              onClick={() => setActiveTab("settings")}
              className={`px-4 py-2 rounded-lg flex items-center gap-2 transition-colors ${
                activeTab === "settings"
                  ? "bg-primary-600 text-white"
                  : "bg-gray-700 hover:bg-gray-600"
              }`}
            >
              <Settings className="w-4 h-4" />
              Settings
            </button>
          </nav>
        </div>
      </header>

      <main className="p-6">
        {activeTab === "dashboard" && (
          <div className="space-y-6">
            {isLoading ? (
              <div className="text-center py-12 text-gray-400">Loading devices...</div>
            ) : (
              <>
                <RoutingMatrix
                  inputs={inputDevices}
                  outputs={outputDevices}
                  routes={routes}
                  onRouteToggle={handleRouteToggle}
                  onVolumeChange={handleVolumeChange}
                  onMuteToggle={handleMuteToggle}
                  onAddRoute={handleAddRoute}
                />

                <div className="grid grid-cols-3 gap-6">
                  <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
                    <h3 className="text-lg font-semibold mb-3">Input Levels</h3>
                    <div className="space-y-4">
                      {inputDevices.map((device) => (
                        <VuMeter key={device.id} level={Math.random() * 60} label={device.name} />
                      ))}
                    </div>
                  </div>

                  <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
                    <h3 className="text-lg font-semibold mb-3">Output Levels</h3>
                    <div className="space-y-4">
                      {outputDevices.map((device) => (
                        <VuMeter key={device.id} level={Math.random() * 60} label={device.name} />
                      ))}
                    </div>
                  </div>

                  <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
                    <h3 className="text-lg font-semibold mb-3">Status</h3>
                    <div className="space-y-2 text-sm text-gray-400">
                      <div className="flex justify-between">
                        <span>Latency:</span>
                        <span className="text-white">5.2 ms</span>
                      </div>
                      <div className="flex justify-between">
                        <span>CPU Usage:</span>
                        <span className="text-white">2.3%</span>
                      </div>
                      <div className="flex justify-between">
                        <span>Sample Rate:</span>
                        <span className="text-white">48000 Hz</span>
                      </div>
                      <div className="flex justify-between">
                        <span>Buffer Size:</span>
                        <span className="text-white">512 samples</span>
                      </div>
                      <div className="flex justify-between">
                        <span>Active Routes:</span>
                        <span className="text-white">{routes.length}</span>
                      </div>
                    </div>
                  </div>
                </div>
              </>
            )}
          </div>
        )}

        {activeTab === "devices" && (
          <div className="space-y-6">
            <div className="grid grid-cols-2 gap-6">
              <DeviceList
                devices={inputDevices}
                title="Input Devices"
                selectedDevice={selectedInput}
                onSelectDevice={setSelectedInput}
              />
              <DeviceList
                devices={outputDevices}
                title="Output Devices"
                selectedDevice={selectedOutput}
                onSelectDevice={setSelectedOutput}
              />
            </div>
            <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
              <h3 className="text-lg font-semibold mb-4">Create Virtual Device</h3>
              <div className="flex gap-4">
                <input
                  type="text"
                  value={newDeviceName}
                  onChange={(e) => setNewDeviceName(e.target.value)}
                  onKeyDown={(e) => e.key === "Enter" && handleCreateDevice()}
                  placeholder="Device name (e.g., VAC-4)"
                  className="flex-1 px-4 py-2 bg-gray-700 rounded-lg border border-gray-600 focus:border-primary-500 focus:outline-none"
                />
                <button
                  onClick={handleCreateDevice}
                  className="px-6 py-2 bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors"
                >
                  Create
                </button>
              </div>
            </div>
          </div>
        )}

        {activeTab === "fx" && (
          <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
            <h2 className="text-xl font-semibold mb-4">FX Chain Editor</h2>
            {routes.length > 0 ? (
              <FxChain
                routeId={routes[0].inputId}
                onDspChange={(settings) => handleDspChange(routes[0].inputId, settings)}
                initialSettings={dspSettings[routes[0].inputId]}
              />
            ) : (
              <p className="text-gray-400">Create a route first to configure its FX chain.</p>
            )}
          </div>
        )}

        {activeTab === "presets" && (
          <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
            <Presets
              presets={presetNames}
              onSave={handleSavePreset}
              onLoad={handleLoadPreset}
              onDelete={handleDeletePreset}
              onExport={handleExportPreset}
              onImport={handleImportPreset}
            />
          </div>
        )}

        {activeTab === "mobile" && (
          <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
            <h2 className="text-xl font-semibold mb-4">Mobile Companion</h2>
            <div className="space-y-4">
              <p className="text-gray-400">Pair with mobile app to use phone microphone as input source.</p>
              <div className="bg-gray-700 rounded-lg p-6 text-center">
                <QrCodePlaceholder />
                <p className="text-sm text-gray-400 mt-4">Scan QR code with mobile app to pair</p>
              </div>
            </div>
          </div>
        )}

        {activeTab === "settings" && (
          <div className="bg-gray-800 rounded-xl p-6 border border-gray-700 max-w-2xl">
            <h2 className="text-xl font-semibold mb-4">Settings</h2>
            <div className="space-y-6">
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Buffer Size</label>
                <select className="w-full px-4 py-2 bg-gray-700 rounded-lg border border-gray-600 focus:border-primary-500 focus:outline-none">
                  <option value="64">64 samples (lowest latency)</option>
                  <option value="128">128 samples</option>
                  <option value="256">256 samples</option>
                  <option value="512">512 samples (recommended)</option>
                  <option value="1024">1024 samples</option>
                  <option value="2048">2048 samples (highest stability)</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Sample Rate</label>
                <select className="w-full px-4 py-2 bg-gray-700 rounded-lg border border-gray-600 focus:border-primary-500 focus:outline-none">
                  <option value="44100">44.1 kHz</option>
                  <option value="48000">48 kHz</option>
                  <option value="96000">96 kHz</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Theme</label>
                <select className="w-full px-4 py-2 bg-gray-700 rounded-lg border border-gray-600 focus:border-primary-500 focus:outline-none">
                  <option value="dark">Dark</option>
                  <option value="light">Light</option>
                  <option value="system">System</option>
                </select>
              </div>
              <div className="flex items-center justify-between">
                <div>
                  <label className="block text-sm font-medium text-gray-300">Start Minimized</label>
                  <p className="text-xs text-gray-400">Launch application in system tray</p>
                </div>
                <button className="w-12 h-6 bg-gray-600 rounded-full relative transition-colors">
                  <div className="w-5 h-5 bg-white rounded-full absolute top-0.5 left-0.5" />
                </button>
              </div>
            </div>
          </div>
        )}
      </main>

      {toast && (
        <div className="fixed right-5 top-5 z-50">
          <div
            className={`px-4 py-3 rounded-lg border shadow-lg ${
              toast.type === "success"
                ? "bg-emerald-600/95 border-emerald-400 text-white"
                : "bg-red-600/95 border-red-400 text-white"
            }`}
          >
            {toast.message}
          </div>
        </div>
      )}
    </div>
  );
}

function QrCodePlaceholder() {
  return (
    <div className="w-24 h-24 bg-white rounded grid grid-cols-5 grid-rows-5 gap-0.5 p-2 mx-auto">
      {Array.from({ length: 25 }).map((_, i) => (
        <div
          key={i}
          className={`rounded-sm ${
            (i === 0 || i === 1 || i === 4 || i === 5 || i === 6 || i === 9 || i === 10 ||
             i === 14 || i === 15 || i === 16 || i === 19 || i === 20 || i === 24)
              ? "bg-black"
              : "bg-gray-200"
          }`}
        />
      ))}
    </div>
  );
}

export default App;
