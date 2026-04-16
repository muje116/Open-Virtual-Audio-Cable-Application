import { useState, useEffect } from "react";
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

interface Route {
  inputId: string;
  outputId: string;
  volume: number;
  muted: boolean;
}

function App() {
  const [activeTab, setActiveTab] = useState("dashboard");
  const [inputDevices, setInputDevices] = useState<Device[]>([]);
  const [outputDevices, setOutputDevices] = useState<Device[]>([]);
  const [routes, setRoutes] = useState<Route[]>([]);
  const [selectedInput, setSelectedInput] = useState<string | null>(null);
  const [selectedOutput, setSelectedOutput] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadDevices();
  }, []);

  const loadDevices = async () => {
    try {
      const devices = await invoke<Device[]>("get_audio_devices");
      const inputs = devices.filter((d) => d.deviceType.includes("Microphone") || d.deviceType.includes("Network"));
      const outputs = devices.filter((d) => d.deviceType.includes("SystemAudio"));
      setInputDevices(inputs);
      setOutputDevices(outputs);
    } catch (error) {
      console.error("Failed to load devices:", error);
      // Set mock data for development when Rust is not available
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
  };

  const handleRouteToggle = (inputId: string, outputId: string) => {
    const existingRoute = routes.find((r) => r.inputId === inputId && r.outputId === outputId);
    if (existingRoute) {
      setRoutes(routes.filter((r) => r !== existingRoute));
    } else {
      setRoutes([...routes, { inputId, outputId, volume: 100, muted: false }]);
    }
  };

  const handleVolumeChange = (inputId: string, outputId: string, volume: number) => {
    setRoutes(
      routes.map((r) =>
        r.inputId === inputId && r.outputId === outputId ? { ...r, volume } : r
      )
    );
  };

  const handleMuteToggle = (inputId: string, outputId: string) => {
    setRoutes(
      routes.map((r) =>
        r.inputId === inputId && r.outputId === outputId ? { ...r, muted: !r.muted } : r
      )
    );
  };

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      <header className="bg-gray-800 border-b border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Radio className="w-8 h-8 text-primary-500" />
            <h1 className="text-2xl font-bold">Virtual Audio Cable</h1>
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
                  placeholder="Device name (e.g., VAC-4)"
                  className="flex-1 px-4 py-2 bg-gray-700 rounded-lg border border-gray-600 focus:border-primary-500 focus:outline-none"
                />
                <button className="px-6 py-2 bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors">
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
              <FxChain routeId={routes[0].inputId} />
            ) : (
              <p className="text-gray-400">Create a route first to configure its FX chain.</p>
            )}
          </div>
        )}

        {activeTab === "presets" && (
          <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
            <Presets />
          </div>
        )}

        {activeTab === "mobile" && (
          <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
            <h2 className="text-xl font-semibold mb-4">Mobile Companion</h2>
            <div className="space-y-4">
              <p className="text-gray-400">Pair with mobile app to use phone microphone as input source.</p>
              <div className="bg-gray-700 rounded-lg p-6 text-center">
                <div className="w-32 h-32 mx-auto mb-4 bg-gray-600 rounded-lg flex items-center justify-center">
                  <QrCodePlaceholder />
                </div>
                <p className="text-sm text-gray-400">Scan QR code with mobile app to pair</p>
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
                  <option value="512" selected>512 samples (recommended)</option>
                  <option value="1024">1024 samples</option>
                  <option value="2048">2048 samples (highest stability)</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Sample Rate</label>
                <select className="w-full px-4 py-2 bg-gray-700 rounded-lg border border-gray-600 focus:border-primary-500 focus:outline-none">
                  <option value="44100">44.1 kHz</option>
                  <option value="48000" selected>48 kHz</option>
                  <option value="96000">96 kHz</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Theme</label>
                <select className="w-full px-4 py-2 bg-gray-700 rounded-lg border border-gray-600 focus:border-primary-500 focus:outline-none">
                  <option value="dark" selected>Dark</option>
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
    </div>
  );
}

function QrCodePlaceholder() {
  return (
    <div className="w-24 h-24 bg-white rounded grid grid-cols-5 grid-rows-5 gap-0.5 p-2">
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
