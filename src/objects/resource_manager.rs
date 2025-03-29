
use std::{collections::HashMap, os::unix::thread, thread::ThreadId};

use super::{ram_emulator::RamEmulator, resource::Resource, resource_tree::ResourceTree};


// find quickly a wich thread is on the three
pub type ThreadMap = HashMap<ThreadId, Resource>;

pub struct ResourceManager {
    ram_emulator: RamEmulator,
    resources_tree: Vec<ResourceTree>,
    thread_map: ThreadMap,
}


impl ResourceManager {
    
    pub fn new() -> ResourceManager {
        ResourceManager {
            ram_emulator: RamEmulator::new(),
            resources_tree: Vec::new(),
            thread_map: HashMap::new(),
        }
    }
    
    pub fn add_resource(&mut self, resource: Resource, thread_id: ThreadId) {

        let resource_tree = ResourceTree::new(resource.clone(), thread_id);
        
        RamEmulator::add_resource(&mut self.ram_emulator, resource.clone());
        
        if let Some(existing_tree) = self.resources_tree.iter_mut().find(|tree| tree.get_thread_id() == thread_id) {
            existing_tree.add_child(resource_tree);
        } else {
            self.resources_tree.push(resource_tree);
        }

        self.thread_map.insert(thread_id, resource);
    }

    pub fn add_ram_resource(&mut self, resource: Resource) {
        self.ram_emulator.add_resource(resource);
    }

    pub fn get_resource_by_thread_id(&self, thread_id: ThreadId) -> Option<&Resource> {
        self.thread_map.get(&thread_id)
    }

}