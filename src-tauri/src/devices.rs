use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Host;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VirtualDevice {
    pub id: String,
    pub name: String,
    pub channels: u16,
    pub sample_rate: u32,
    pub is_active: bool,
}

pub struct DeviceManager {
    virtual_devices: HashMap<String, VirtualDevice>,
    next_device_id: usize,
    host: Host,
}

impl DeviceManager {
    pub fn new() -> Self {
        let host = cpal::default_host();
        DeviceManager {
            virtual_devices: HashMap::new(),
            next_device_id: 1,
            host,
        }
    }

    pub fn detect_virtual_devices(&mut self) -> Result<Vec<VirtualDevice>> {
        let mut detected_devices = Vec::new();
        
        // Detect virtual audio devices (VB-Cable, etc.)
        if let Ok(output_devices) = self.host.output_devices() {
            for (index, device) in output_devices.enumerate() {
                if let Ok(name) = device.name() {
                    // Check for common virtual audio device names
                    let is_virtual = name.contains("VB-Cable") || 
                                    name.contains("Virtual") || 
                                    name.contains("VAC") ||
                                    name.contains("Cable");
                    
                    if is_virtual {
                        if let Ok(default_config) = device.default_output_config() {
                            let id = format!("virtual_{}", index);
                            let virtual_device = VirtualDevice {
                                id: id.clone(),
                                name,
                                channels: default_config.channels(),
                                sample_rate: default_config.sample_rate().0,
                                is_active: true,
                            };
                            self.virtual_devices.insert(id.clone(), virtual_device.clone());
                            detected_devices.push(virtual_device);
                        }
                    }
                }
            }
        }
        
        Ok(detected_devices)
    }

    pub fn create_virtual_device(&mut self, name: String, channels: u16) -> VirtualDevice {
        let id = format!("vac_{}", self.next_device_id);
        self.next_device_id += 1;

        let device = VirtualDevice {
            id: id.clone(),
            name,
            channels,
            sample_rate: 48000,
            is_active: true,
        };

        self.virtual_devices.insert(id.clone(), device.clone());
        device
    }

    pub fn delete_virtual_device(&mut self, id: &str) -> Result<()> {
        self.virtual_devices
            .remove(id)
            .ok_or_else(|| anyhow::anyhow!("Device not found"))?;
        Ok(())
    }

    pub fn get_virtual_devices(&self) -> Vec<VirtualDevice> {
        self.virtual_devices.values().cloned().collect()
    }

    pub fn get_virtual_device(&self, id: &str) -> Option<VirtualDevice> {
        self.virtual_devices.get(id).cloned()
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}
