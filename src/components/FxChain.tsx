import { useState } from "react";
import { Sliders, Gauge, Zap } from "lucide-react";

interface FxChainProps {
  routeId: string;
}

export function FxChain({ }: FxChainProps) {
  const [gain, setGain] = useState(100);
  const [noiseGateEnabled, setNoiseGateEnabled] = useState(false);
  const [noiseGateThreshold, setNoiseGateThreshold] = useState(-60);
  const [eqBands, setEqBands] = useState([0, 0, 0, 0, 0]);
  const [compressorEnabled, setCompressorEnabled] = useState(false);

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-2">
        <Sliders className="w-5 h-5 text-primary-400" />
        <h3 className="text-lg font-semibold">FX Chain</h3>
      </div>

      {/* Gain */}
      <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
        <div className="flex items-center gap-2 mb-3">
          <Gauge className="w-4 h-4 text-primary-400" />
          <h4 className="font-medium">Gain</h4>
        </div>
        <div className="space-y-2">
          <input
            type="range"
            min="0"
            max="200"
            value={gain}
            onChange={(e) => setGain(parseInt(e.target.value))}
            className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-primary-500"
          />
          <div className="flex justify-between text-sm text-gray-400">
            <span>0%</span>
            <span className="text-white font-medium">{gain}%</span>
            <span>200%</span>
          </div>
        </div>
      </div>

      {/* Noise Gate */}
      <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-2">
            <Zap className="w-4 h-4 text-primary-400" />
            <h4 className="font-medium">Noise Gate</h4>
          </div>
          <button
            onClick={() => setNoiseGateEnabled(!noiseGateEnabled)}
            className={`px-3 py-1 rounded-full text-xs font-medium transition-colors ${
              noiseGateEnabled
                ? "bg-primary-600 text-white"
                : "bg-gray-700 text-gray-400"
            }`}
          >
            {noiseGateEnabled ? "ON" : "OFF"}
          </button>
        </div>
        {noiseGateEnabled && (
          <div className="space-y-2">
            <label className="text-sm text-gray-400">Threshold</label>
            <input
              type="range"
              min="-100"
              max="0"
              value={noiseGateThreshold}
              onChange={(e) => setNoiseGateThreshold(parseInt(e.target.value))}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-primary-500"
            />
            <div className="text-center text-sm text-gray-400">
              {noiseGateThreshold} dB
            </div>
          </div>
        )}
      </div>

      {/* Equalizer */}
      <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
        <div className="flex items-center gap-2 mb-3">
          <Sliders className="w-4 h-4 text-primary-400" />
          <h4 className="font-medium">5-Band EQ</h4>
        </div>
        <div className="grid grid-cols-5 gap-4">
          {["60Hz", "250Hz", "1kHz", "4kHz", "16kHz"].map((freq, index) => (
            <div key={freq} className="space-y-2">
              <label className="text-xs text-gray-400 block text-center">
                {freq}
              </label>
              <input
                type="range"
                min="-12"
                max="12"
                value={eqBands[index]}
                onChange={(e) => {
                  const newBands = [...eqBands];
                  newBands[index] = parseInt(e.target.value);
                  setEqBands(newBands);
                }}
                className="w-full h-20 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-primary-500"
                style={{
                  writingMode: "vertical-lr",
                  direction: "rtl",
                }}
              />
              <div className="text-center text-xs text-gray-400">
                {eqBands[index]} dB
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Compressor */}
      <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-2">
            <Gauge className="w-4 h-4 text-primary-400" />
            <h4 className="font-medium">Compressor</h4>
          </div>
          <button
            onClick={() => setCompressorEnabled(!compressorEnabled)}
            className={`px-3 py-1 rounded-full text-xs font-medium transition-colors ${
              compressorEnabled
                ? "bg-primary-600 text-white"
                : "bg-gray-700 text-gray-400"
            }`}
          >
            {compressorEnabled ? "ON" : "OFF"}
          </button>
        </div>
        {compressorEnabled && (
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <label className="text-sm text-gray-400">Threshold</label>
              <input
                type="range"
                min="-60"
                max="0"
                defaultValue="-20"
                className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-primary-500"
              />
            </div>
            <div className="space-y-2">
              <label className="text-sm text-gray-400">Ratio</label>
              <input
                type="range"
                min="1"
                max="20"
                defaultValue="4"
                className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-primary-500"
              />
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
