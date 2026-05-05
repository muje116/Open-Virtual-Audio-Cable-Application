import { Volume2, VolumeX, Plus } from "lucide-react";

interface Route {
  inputId: string;
  outputId: string;
  volume: number;
  muted: boolean;
}

interface RoutingMatrixProps {
  inputs: Array<{ id: string; name: string }>;
  outputs: Array<{ id: string; name: string }>;
  routes: Route[];
  onRouteToggle: (inputId: string, outputId: string) => void;
  onVolumeChange: (inputId: string, outputId: string, volume: number) => void;
  onMuteToggle: (inputId: string, outputId: string) => void;
  onAddRoute: () => void;
}

export function RoutingMatrix({
  inputs,
  outputs,
  routes,
  onRouteToggle,
  onVolumeChange,
  onMuteToggle,
  onAddRoute,
}: RoutingMatrixProps) {
  const getRoute = (inputId: string, outputId: string) => {
    return routes.find((r) => r.inputId === inputId && r.outputId === outputId);
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold">Routing Matrix</h3>
        <button
          onClick={onAddRoute}
          className="flex items-center gap-2 px-4 py-2 bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors"
        >
          <Plus className="w-4 h-4" />
          Add Route
        </button>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr>
              <th className="text-left p-3 bg-gray-700 rounded-tl-lg">Input / Output</th>
              {outputs.map((output) => (
                <th key={output.id} className="p-3 bg-gray-700 min-w-[120px]">
                  <div className="flex items-center gap-2 justify-center">
                    <div className="w-2 h-2 rounded-full bg-green-500" />
                    {output.name}
                  </div>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {inputs.map((input) => (
              <tr key={input.id}>
                <td className="p-3 bg-gray-800 font-medium">{input.name}</td>
                {outputs.map((output) => {
                  const route = getRoute(input.id, output.id);
                  const isActive = !!route;

                  return (
                    <td key={output.id} className="p-2 bg-gray-800/50">
                      <div
                        className={`p-3 rounded-lg border-2 transition-all ${
                          isActive
                            ? "border-primary-500 bg-primary-500/10"
                            : "border-gray-600 hover:border-gray-500"
                        }`}
                      >
                        {isActive ? (
                          <div className="space-y-2">
                            <div className="flex items-center justify-between">
                              <span className="text-xs text-gray-400">Active</span>
                              <button
                                onClick={() => onMuteToggle(input.id, output.id)}
                                className={`p-1 rounded ${
                                  route.muted
                                    ? "bg-red-500/20 text-red-400"
                                    : "bg-gray-700 hover:bg-gray-600"
                                }`}
                              >
                                {route.muted ? (
                                  <VolumeX className="w-4 h-4" />
                                ) : (
                                  <Volume2 className="w-4 h-4" />
                                )}
                              </button>
                            </div>
                            <input
                              type="range"
                              min="0"
                              max="100"
                              value={route.volume}
                              onChange={(e) =>
                                onVolumeChange(
                                  input.id,
                                  output.id,
                                  parseInt(e.target.value)
                                )
                              }
                              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-primary-500"
                            />
                            <div className="text-xs text-center text-gray-400">
                              {route.volume}%
                            </div>
                          </div>
                        ) : (
                          <button
                            onClick={() => onRouteToggle(input.id, output.id)}
                            className="w-full h-full flex items-center justify-center text-gray-500 hover:text-primary-400 transition-colors"
                          >
                            <Plus className="w-5 h-5" />
                          </button>
                        )}
                      </div>
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
