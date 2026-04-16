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
