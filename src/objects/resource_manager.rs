
use std::{collections::HashMap, thread::ThreadId};

use super::{ram_emulator::RamEmulator, resource::Resource, resource_tree::ResourceTree};


// find quickly a wich thread is on the three
pub type ThreadMap = HashMap<ThreadId, Resource>;

pub struct ResourceManager {
    ram_emulator: RamEmulator,
    resources_tree: Vec<ResourceTree>,
    thread_map: ThreadMap,
}