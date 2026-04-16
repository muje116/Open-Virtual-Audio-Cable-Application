interface VuMeterProps {
  level: number; // 0-100
  label: string;
}

export function VuMeter({ level, label }: VuMeterProps) {
  return (
    <div className="space-y-2">
      <div className="flex justify-between text-sm">
        <span className="text-gray-400">{label}</span>
        <span className="text-gray-300">{level.toFixed(1)} dB</span>
      </div>
      <div className="relative h-4 bg-gray-700 rounded-full overflow-hidden">
        <div
          className="absolute top-0 left-0 h-full transition-all duration-100 ease-linear"
          style={{
            width: `${Math.min(level, 100)}%`,
            background: level > 80
              ? "linear-gradient(to right, #22c55e 0%, #22c55e 60%, #eab308 60%, #eab308 80%, #ef4444 80%)"
              : level > 60
              ? "linear-gradient(to right, #22c55e 0%, #22c55e 60%, #eab308 60%)"
              : "#22c55e",
          }}
        />
      </div>
    </div>
  );
}
