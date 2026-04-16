use anyhow::Result;
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
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            virtual_devices: HashMap::new(),
            next_device_id: 1,
        }
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
