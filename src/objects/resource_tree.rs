use std::{collections::HashMap, thread::ThreadId};

use super::resource::Resource;

pub struct ResourceTree{
    resource: Resource,
    root: Node,
    children: Vec<ResourceTree>,
}

pub struct Node {
    thread_id: ThreadId,
    children: Vec<Node>,
}