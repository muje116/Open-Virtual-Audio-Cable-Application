import { useState } from "react";
import { Save, FolderOpen, Download, Upload, Trash2 } from "lucide-react";

interface PresetsProps {
  presets: string[];
  onSave: (name: string) => Promise<void> | void;
  onLoad: (name: string) => Promise<void> | void;
  onDelete: (name: string) => Promise<void> | void;
  onExport: (name: string) => Promise<void> | void;
  onImport: (file: File) => Promise<void> | void;
}

export function Presets({ presets, onSave, onLoad, onDelete, onExport, onImport }: PresetsProps) {
  const [selectedPreset, setSelectedPreset] = useState<string | null>(null);
  const [newPresetName, setNewPresetName] = useState("");

  const handleSave = async () => {
    const name = newPresetName.trim();
    if (!name) return;
    await onSave(name);
    setNewPresetName("");
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between gap-3">
        <h3 className="text-lg font-semibold">Presets</h3>
        <div className="flex items-center gap-3">
          <input
            value={newPresetName}
            onChange={(e) => setNewPresetName(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSave()}
            placeholder="Preset name"
            className="px-3 py-2 bg-gray-700 rounded-lg border border-gray-600 focus:border-primary-500 focus:outline-none"
          />
          <button
            onClick={handleSave}
            className="flex items-center gap-2 px-4 py-2 bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors"
          >
            <Save className="w-4 h-4" />
            Save Current
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {presets.map((preset) => (
          <div
            key={preset}
            onClick={() => setSelectedPreset(preset)}
            className={`p-4 rounded-xl border-2 cursor-pointer transition-all ${
              selectedPreset === preset
                ? "border-primary-500 bg-primary-500/10"
                : "border-gray-700 bg-gray-800 hover:border-gray-600"
            }`}
          >
            <div className="flex items-start justify-between mb-2">
              <h4 className="font-medium">{preset}</h4>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onDelete(preset);
                  if (selectedPreset === preset) setSelectedPreset(null);
                }}
                className="p-1 hover:bg-gray-700 rounded transition-colors"
              >
                <Trash2 className="w-4 h-4 text-gray-400 hover:text-red-400" />
              </button>
            </div>
          </div>
        ))}
      </div>

      <div className="flex gap-4">
        <button
          onClick={() => selectedPreset && onLoad(selectedPreset)}
          className="flex items-center gap-2 px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors disabled:opacity-50"
          disabled={!selectedPreset}
        >
          <FolderOpen className="w-4 h-4" />
          Load Selected
        </button>
        <button
          onClick={() => selectedPreset && onExport(selectedPreset)}
          className="flex items-center gap-2 px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors disabled:opacity-50"
          disabled={!selectedPreset}
        >
          <Download className="w-4 h-4" />
          Export Preset
        </button>
        <label className="flex items-center gap-2 px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors cursor-pointer">
          <Upload className="w-4 h-4" />
          Import Preset
          <input
            type="file"
            accept="application/json,.json"
            className="hidden"
            onChange={(e) => {
              const file = e.target.files?.[0];
              if (file) onImport(file);
              e.currentTarget.value = "";
            }}
          />
        </label>
      </div>
    </div>
  );
}
