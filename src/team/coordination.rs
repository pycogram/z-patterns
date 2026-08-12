use agent_core::AgentId;

/// Team coordination mechanism
#[derive(Debug, Clone)]
pub struct Coordination {
    leader: Option<AgentId>,
    members: Vec<AgentId>,
}

impl Coordination {
    /// Create new coordination
    pub fn new() -> Self {
        Self {
            leader: None,
            members: Vec::new(),
        }
    }

    /// Set leader
    pub fn set_leader(&mut self, leader: AgentId) {
        self.leader = Some(leader);
    }

    /// Add member
    pub fn add_member(&mut self, member: AgentId) {
        self.members.push(member);
    }

    /// Get leader
    pub fn leader(&self) -> Option<&AgentId> {
        self.leader.as_ref()
    }

    /// Get members
    pub fn members(&self) -> &[AgentId] {
        &self.members
    }
}

impl Default for Coordination {
    fn default() -> Self {
        Self::new()
    }
}
