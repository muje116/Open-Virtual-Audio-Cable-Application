import { useState } from "react";
import { Save, FolderOpen, Download, Upload, Trash2 } from "lucide-react";

interface Preset {
  name: string;
  createdAt: string;
  routes: number;
}

export function Presets() {
  const [presets, setPresets] = useState<Preset[]>([
    { name: "Default Setup", createdAt: "2024-01-15", routes: 2 },
    { name: "Podcast Recording", createdAt: "2024-01-20", routes: 4 },
    { name: "Gaming Stream", createdAt: "2024-02-01", routes: 3 },
  ]);
  const [selectedPreset, setSelectedPreset] = useState<string | null>(null);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold">Presets</h3>
        <button className="flex items-center gap-2 px-4 py-2 bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors">
          <Save className="w-4 h-4" />
          Save Current as Preset
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {presets.map((preset) => (
          <div
            key={preset.name}
            onClick={() => setSelectedPreset(preset.name)}
            className={`p-4 rounded-xl border-2 cursor-pointer transition-all ${
              selectedPreset === preset.name
                ? "border-primary-500 bg-primary-500/10"
                : "border-gray-700 bg-gray-800 hover:border-gray-600"
            }`}
          >
            <div className="flex items-start justify-between mb-2">
              <h4 className="font-medium">{preset.name}</h4>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  setPresets(presets.filter((p) => p.name !== preset.name));
                }}
                className="p-1 hover:bg-gray-700 rounded transition-colors"
              >
                <Trash2 className="w-4 h-4 text-gray-400 hover:text-red-400" />
              </button>
            </div>
            <div className="text-sm text-gray-400 space-y-1">
              <div>Created: {preset.createdAt}</div>
              <div>Routes: {preset.routes}</div>
            </div>
          </div>
        ))}

        <div className="p-4 rounded-xl border-2 border-dashed border-gray-700 bg-gray-800/50 flex items-center justify-center min-h-[120px] cursor-pointer hover:border-gray-600 transition-colors">
          <div className="text-center text-gray-400">
            <Save className="w-8 h-8 mx-auto mb-2" />
            <span className="text-sm">Create New Preset</span>
          </div>
        </div>
      </div>

      <div className="flex gap-4">
        <button className="flex items-center gap-2 px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors">
          <FolderOpen className="w-4 h-4" />
          Load Selected
        </button>
        <button className="flex items-center gap-2 px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors">
          <Download className="w-4 h-4" />
          Export Preset
        </button>
        <button className="flex items-center gap-2 px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors">
          <Upload className="w-4 h-4" />
          Import Preset
        </button>
      </div>
    </div>
  );
}
