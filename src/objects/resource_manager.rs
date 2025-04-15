use std::{collections::HashMap, sync::Arc, thread::ThreadId};

use super::{ram_emulator::RamEmulator, resource::Resource, resource_tree::ResourceTree};

pub type ThreadMap = HashMap<ThreadId, Arc<Resource>>;

pub struct ResourceManager {
    ram_emulator: RamEmulator,
    pub resources_tree: Vec<ResourceTree>,
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

    pub fn add_resource(&mut self, resource: Arc<Resource>, thread_id: ThreadId) -> Result<(), String> {
        let resource_name = resource.name.clone();

        // Tenta bloquear o recurso
        //resource.lock();

        // Adiciona o recurso à árvore de recursos
        let resource_tree = ResourceTree::new(Arc::new(resource.copy(resource_name.clone())), thread_id);
        //let resource_tree = ResourceTree::new(resource.clone(), thread_id);

        if let Some(existing_tree) = self.resources_tree.iter_mut().find(|tree| tree.get_thread_id() == thread_id) {
            existing_tree.add_child(resource_tree);
        } else {
            self.resources_tree.push(resource_tree);
        }

        self.thread_map.insert(thread_id, resource);

        println!("Thread {:?} alocou o recurso '{}'.", thread_id, resource_name);

        Ok(())
    }

    pub fn release_resource(&mut self, thread_id: ThreadId) {
        if let Some(resource) = self.thread_map.remove(&thread_id) {
            resource.unlock();
            println!("Thread {:?} liberou o recurso '{}'.", thread_id, resource.name);
        } else {
            println!("Thread {:?} tentou liberar um recurso que não possui.", thread_id);
        }
    }

    pub fn release_all(&mut self) {
        for (thread_id, resource) in &self.thread_map {
            resource.unlock();
            println!("Thread {:?} liberou o recurso '{}'.", thread_id, resource.name);
        }
        self.thread_map.clear();
    }

    pub fn detect_deadlock(&self) -> bool {
        let mut visited = HashMap::new();
        let mut stack = HashMap::new();

        for resource_tree in &self.resources_tree {
            if self.has_cycle(resource_tree, &mut visited, &mut stack) {
                println!("Deadlock detectado!");
                return true;
            }
        }

        println!("Nenhum deadlock detectado.");
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
            return true; // Ciclo detectado
        }

        if visited.get(&thread_id).copied().unwrap_or(false) {
            return false;
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