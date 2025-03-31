
#[derive(Debug, Clone)]
pub struct Resource {
    pub name: String,
    pub path: String,
    pub size: u64,
}

impl Resource {
    pub fn new(name: String) -> Resource {
        
        let path = format!("/resources/{}", name);
        let size = 0; // Default size, can be set later
        
        Resource { 
            name, 
            path, 
            size 
        }
    }
}