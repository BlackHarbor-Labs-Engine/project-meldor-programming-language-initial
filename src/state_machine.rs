use std::collections::HashMap;

// Renamed state to _state to satisfy unused variable warning
fn process_state_machine(_state: &HashMap<String, String>) -> Result<(), String> {
    // Implementation goes here
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceState {
    Idle,
    Allocated,
    InUse,
    Released,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub id: u32,
    pub state: ResourceState,
    pub data: Option<String>,
}

impl Resource {
    pub fn new(id: u32) -> Self {
        Resource {
            id,
            state: ResourceState::Idle,
            data: None,
        }
    }
}

pub struct ResourceManager {
    resources: HashMap<u32, Resource>,
    next_id: u32,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn allocate(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.resources.insert(id, Resource::new(id));
        id
    }

    // Hardened acquire: returns immediately on invalid state
    pub fn acquire(&mut self, id: u32) -> Result<(), String> {
        let resource = self.resources.get_mut(&id).ok_or_else(|| format!("Resource {} does not exist", id))?;
        
        if resource.state == ResourceState::Idle {
            resource.state = ResourceState::InUse;
            Ok(())
        } else {
            Err(format!("Resource {} is not idle, current state: {:?}", id, resource.state))
        }
    }

    // Hardened release: returns immediately on invalid state
    pub fn release(&mut self, id: u32) -> Result<(), String> {
        let resource = self.resources.get_mut(&id).ok_or_else(|| format!("Resource {} does not exist", id))?;
        
        if resource.state == ResourceState::InUse {
            resource.state = ResourceState::Released;
            Ok(())
        } else {
            Err(format!("Resource {} is not in use, current state: {:?}", id, resource.state))
        }
    }

    // Deallocate is safe if state is Released, otherwise it's an error.
    pub fn deallocate(&mut self, id: u32) -> Result<(), String> {
        let resource = self.resources.get_mut(&id).ok_or_else(|| format!("Resource {} does not exist", id))?;
        
        if resource.state == ResourceState::Released {
            self.resources.remove(&id);
            Ok(())
        } else {
            Err(format!("Resource {} is not released, current state: {:?}", id, resource.state))
        }
    }

    pub fn get_state(&self, id: u32) -> Option<ResourceState> {
        self.resources.get(&id).map(|r| r.state)
    }

    pub fn set_data(&mut self, id: u32, data: String) -> Result<(), String> {
        let resource = self.resources.get_mut(&id).ok_or_else(|| format!("Resource {} does not exist", id))?;
        resource.data = Some(data);
        Ok(())
    }
}
