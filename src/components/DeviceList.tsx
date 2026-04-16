import { Mic, Radio, Volume2, Smartphone } from "lucide-react";

interface Device {
  id: string;
  name: string;
  deviceType: string;
  sampleRate: number;
  channels: number;
}

interface DeviceListProps {
  devices: Device[];
  title: string;
  selectedDevice: string | null;
  onSelectDevice: (deviceId: string) => void;
}

const getDeviceIcon = (deviceType: string) => {
  if (deviceType.includes("Microphone")) return Mic;
  if (deviceType.includes("SystemAudio")) return Radio;
  if (deviceType.includes("AudioFile")) return Volume2;
  if (deviceType.includes("NetworkStream")) return Smartphone;
  return Mic;
};

export function DeviceList({
  devices,
  title,
  selectedDevice,
  onSelectDevice,
}: DeviceListProps) {
  return (
    <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
      <h3 className="text-lg font-semibold mb-4">{title}</h3>
      {devices.length === 0 ? (
        <p className="text-gray-400 text-sm">No devices detected</p>
      ) : (
        <div className="space-y-2">
          {devices.map((device) => {
            const Icon = getDeviceIcon(device.deviceType);
            const isSelected = selectedDevice === device.id;

            return (
              <button
                key={device.id}
                onClick={() => onSelectDevice(device.id)}
                className={`w-full p-4 rounded-lg border-2 transition-all text-left ${
                  isSelected
                    ? "border-primary-500 bg-primary-500/10"
                    : "border-gray-600 hover:border-gray-500 bg-gray-700/30"
                }`}
              >
                <div className="flex items-center gap-3">
                  <Icon className="w-5 h-5 text-primary-400" />
                  <div className="flex-1">
                    <div className="font-medium">{device.name}</div>
                    <div className="text-xs text-gray-400 mt-1">
                      {device.sampleRate} Hz • {device.channels} channels
                    </div>
                  </div>
                  {isSelected && (
                    <div className="w-3 h-3 rounded-full bg-green-500" />
                  )}
                </div>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}
