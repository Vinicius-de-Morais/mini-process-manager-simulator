use std::collections::HashMap;

use super::resource::Resource;


pub struct RamEmulator {
    resources: HashMap<String, Resource>,
}

