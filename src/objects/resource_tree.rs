use std::{collections::HashMap, sync::Arc, thread::ThreadId};

use super::resource::Resource;

pub struct ResourceTree{
    resource: Arc<Resource>,
    root: Node,
    children: Vec<ResourceTree>,
    waiting_threads: Vec<ThreadId>,
}

impl ResourceTree {
    pub fn new(resource: Arc<Resource>, thread_id: ThreadId) -> ResourceTree {
        ResourceTree {
            resource,
            root: Node {
                thread_id: thread_id,
                children: Vec::new(),
            },
            children: Vec::new(),
            waiting_threads: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: ResourceTree) {
        self.children.push(child);
    }

    pub fn add_node(&mut self, node: Node) {
        self.root.children.push(node);
    }

    pub fn get_resource(&self) -> &Resource {
        &self.resource
    }
    pub fn get_root(&self) -> &Node {
        &self.root
    }

    pub fn get_children(&self) -> &Vec<ResourceTree> {
        &self.children
    }

    pub fn get_thread_id(&self) -> ThreadId {
        self.root.thread_id
    }

    pub fn get_children_thread_ids(&self) -> Vec<ThreadId> {
        self.root.children.iter().map(|child| child.thread_id).collect()
    }

    pub fn add_waiting_thread(&mut self, thread_id: ThreadId) {
        // Adiciona a thread à lista de threads aguardando este recurso
        self.waiting_threads.push(thread_id);
    }

    pub fn get_waiting_threads(&self) -> Option<&Vec<ThreadId>> {
        if self.waiting_threads.is_empty() {
            None
        } else {
            Some(&self.waiting_threads)
        }
    }
}

pub struct Node {
    thread_id: ThreadId,
    children: Vec<Node>,
}