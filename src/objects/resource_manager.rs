
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

    pub fn detect_deadlock(&self) -> bool {
        let mut visited = HashMap::new();
        let mut stack = HashMap::new();

        for resource_tree in &self.resources_tree {
            if self.has_cycle(resource_tree, &mut visited, &mut stack) {
                return true;
            }
        }
        
        false 
    }

    /// Helper function to perform DFS and detect cycles.
    fn has_cycle(
        &self,
        node: &ResourceTree,
        visited: &mut HashMap<ThreadId, bool>,
        stack: &mut HashMap<ThreadId, bool>,
    ) -> bool {
        let thread_id = node.get_thread_id();

        if stack.get(&thread_id).copied().unwrap_or(false) {
            return true; // Cycle detected
        }

        if visited.get(&thread_id).copied().unwrap_or(false) {
            return false; // Already processed, no cycle
        }

        visited.insert(thread_id, true);
        stack.insert(thread_id, true);

        for child in node.get_children() {
            if self.has_cycle(child, visited, stack) {
                return true;
            }
        }

        stack.insert(thread_id, false);
        false
    }

}