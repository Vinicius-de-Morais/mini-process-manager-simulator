use std::collections::HashMap;

use super::resource::Resource;


pub struct RamEmulator {
    resources: HashMap<String, Resource>,
}

impl RamEmulator {
    pub fn new() -> RamEmulator {
        RamEmulator {
            resources: HashMap::new(),
        }
    }

    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.insert(resource.name.clone(), resource);
    }

    pub fn get_resource(&self, name: &str) -> Option<&Resource> {
        self.resources.get(name)
    }

    pub fn remove_resource(&mut self, name: &str) {
        self.resources.remove(name);
    }

}

