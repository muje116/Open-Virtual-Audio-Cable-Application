use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub input_id: String,
    pub output_id: String,
    pub volume: f32,
    pub muted: bool,
    pub dsp: DspConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DspConfig {
    pub gain: f32,
    pub noise_gate_enabled: bool,
    pub noise_gate_threshold: f32,
    pub eq_bands: [f32; 5],
    pub compressor_enabled: bool,
}

impl Default for DspConfig {
    fn default() -> Self {
        DspConfig {
            gain: 1.0,
            noise_gate_enabled: false,
            noise_gate_threshold: 0.01,
            eq_bands: [0.0; 5],
            compressor_enabled: false,
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Route {
            id: String::new(),
            input_id: String::new(),
            output_id: String::new(),
            volume: 1.0,
            muted: false,
            dsp: DspConfig::default(),
        }
    }
}

pub struct RoutingMatrix {
    routes: HashMap<String, Route>,
    next_route_id: usize,
}

impl RoutingMatrix {
    pub fn new() -> Self {
        RoutingMatrix {
            routes: HashMap::new(),
            next_route_id: 1,
        }
    }

    pub fn add_route(&mut self, input_id: String, output_id: String) -> Route {
        let id = format!("route_{}", self.next_route_id);
        self.next_route_id += 1;

        let route = Route {
            id: id.clone(),
            input_id,
            output_id,
            volume: 1.0,
            muted: false,
            dsp: DspConfig::default(),
        };

        self.routes.insert(id.clone(), route.clone());
        route
    }

    pub fn remove_route(&mut self, route_id: &str) -> Option<Route> {
        self.routes.remove(route_id)
    }

    pub fn get_route(&self, route_id: &str) -> Option<&Route> {
        self.routes.get(route_id)
    }

    pub fn update_route(&mut self, route: Route) -> Result<(), String> {
        if !self.routes.contains_key(&route.id) {
            return Err("Route not found".to_string());
        }
        self.routes.insert(route.id.clone(), route);
        Ok(())
    }

    pub fn get_routes_for_input(&self, input_id: &str) -> Vec<Route> {
        self.routes
            .values()
            .filter(|r| r.input_id == input_id)
            .cloned()
            .collect()
    }

    pub fn get_routes_for_output(&self, output_id: &str) -> Vec<Route> {
        self.routes
            .values()
            .filter(|r| r.output_id == output_id)
            .cloned()
            .collect()
    }

    pub fn get_all_routes(&self) -> Vec<Route> {
        self.routes.values().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.routes.clear();
        self.next_route_id = 1;
    }
}

impl Default for RoutingMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_route() {
        let mut matrix = RoutingMatrix::new();
        let route = matrix.add_route("mic_1".into(), "out_1".into());
        assert_eq!(route.input_id, "mic_1");
        assert_eq!(route.output_id, "out_1");
        assert_eq!(route.volume, 1.0);
        assert!(!route.muted);

        let all = matrix.get_all_routes();
        assert_eq!(all.len(), 1);
    }

    #[test]
    fn test_remove_route() {
        let mut matrix = RoutingMatrix::new();
        let route = matrix.add_route("mic_1".into(), "out_1".into());
        let removed = matrix.remove_route(&route.id);
        assert!(removed.is_some());
        assert!(matrix.get_all_routes().is_empty());
    }

    #[test]
    fn test_update_route() {
        let mut matrix = RoutingMatrix::new();
        let mut route = matrix.add_route("mic_1".into(), "out_1".into());
        route.volume = 0.5;
        route.muted = true;
        matrix.update_route(route.clone()).unwrap();

        let stored = matrix.get_route(&route.id).unwrap();
        assert_eq!(stored.volume, 0.5);
        assert!(stored.muted);
    }

    #[test]
    fn test_get_routes_for_input() {
        let mut matrix = RoutingMatrix::new();
        matrix.add_route("mic_1".into(), "out_1".into());
        matrix.add_route("mic_1".into(), "out_2".into());
        matrix.add_route("mic_2".into(), "out_1".into());

        let routes = matrix.get_routes_for_input("mic_1");
        assert_eq!(routes.len(), 2);

        let routes = matrix.get_routes_for_input("mic_3");
        assert_eq!(routes.len(), 0);
    }

    #[test]
    fn test_clear() {
        let mut matrix = RoutingMatrix::new();
        matrix.add_route("a".into(), "b".into());
        matrix.add_route("c".into(), "d".into());
        matrix.clear();
        assert!(matrix.get_all_routes().is_empty());
    }

    #[test]
    fn test_update_nonexistent_route_fails() {
        let mut matrix = RoutingMatrix::new();
        let route = Route::default();
        let result = matrix.update_route(route);
        assert!(result.is_err());
    }
}
