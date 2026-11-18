use std::collections::HashMap;

fn process_state_machine(state: &HashMap<String, String>) -> Result<(), String> {
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

    pub fn acquire(&mut self, id: u32) -> Result<(), String> {
        match self.resources.get_mut(&id) {
            Some(resource) => {
                if resource.state == ResourceState::Idle {
                    resource.state = ResourceState::InUse;
                    Ok(())
                } else {
                    Err(format!("Resource {} is not idle", id))
                }
            } else {
                Err(format!("Resource {} does not exist", id)),
            }
        }
    }

    pub fn release(&mut self, id: u32) -> Result<(), String> {
        match self.resources.get_mut(&id) {
            Some(resource) => {
                if resource.state == ResourceState::InUse {
                    resource.state = ResourceState::Released;
                    Ok(())
                } else {
                    Err(format!("Resource {} is not in use", id))
                }
            } else {
                Err(format!("Resource {} does not exist", id)),
            }
        }
    }

    pub fn deallocate(&mut self, id: u32) -> Result<(), String> {
        match self.resources.get_mut(&id) {
            Some(resource) => {
                if resource.state == ResourceState::Released {
                    self.resources.remove(&id);
                    Ok(())
                } else {
                    Err(format!("Resource {} is not released", id))
                }
            } else {
                Err(format!("Resource {} does not exist", id)),
            }
        }
    }

    pub fn get_state(&self, id: u32) -> Option<ResourceState> {
        self.resources.get(&id).map(|r| r.state)
    }

    pub fn set_data(&mut self, id: u32, data: String) -> Result<(), String> {
        match self.resources.get_mut(&id) {
            Some(resource) => {
                resource.data = Some(data);
                Ok(())
            }
            None => Err(format!("Resource {} does not exist", id)),
        }
    }
}
