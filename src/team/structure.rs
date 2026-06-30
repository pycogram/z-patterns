use super::{Coordination, Role};
use z_core::{Agent, AgentId};
use z_runtime::{Runtime, supervisor::RestartPolicy};
use crate::PatternError;
use std::collections::HashMap;

/// Team structure
#[derive(Debug, Clone)]
pub struct Team {
    name: String,
    roles: HashMap<AgentId, Role>,
    coordination: Coordination,
}

impl Team {
    /// Create a new team
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            roles: HashMap::new(),
            coordination: Coordination::new(),
        }
    }

    /// Assign role to agent
    pub fn assign_role(&mut self, agent_id: AgentId, role: Role) {
        self.roles.insert(agent_id, role);
        self.coordination.add_member(agent_id);
    }

    /// Get agent's role
    pub fn get_role(&self, agent_id: &AgentId) -> Option<&Role> {
        self.roles.get(agent_id)
    }

    /// Set team leader
    pub fn set_leader(&mut self, leader: AgentId) {
        self.coordination.set_leader(leader);
    }

    /// Get team name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get coordination
    pub fn coordination(&self) -> &Coordination {
        &self.coordination
    }

    /// Get all members
    pub fn members(&self) -> Vec<&AgentId> {
        self.roles.keys().collect()
    }

    /// Spawn agents into `runtime`, assign each a role, and return their IDs.
    /// Each tuple is `(agent, name, role)`. The first agent with `RoleType::Leader`
    /// is automatically set as team leader.
    pub async fn spawn_agents(
        &mut self,
        runtime: &Runtime,
        agents: Vec<(Box<dyn Agent>, String, Role)>,
        policy: RestartPolicy,
    ) -> Result<Vec<AgentId>, PatternError> {
        use super::RoleType;
        let mut ids = Vec::with_capacity(agents.len());
        for (agent, name, role) in agents {
            let id = runtime
                .spawn_with_policy(agent, name, policy.clone())
                .await
                .map_err(|e| PatternError::SpawnFailed(e.to_string()))?;
            if role.role_type() == RoleType::Leader && self.coordination().leader().is_none() {
                self.set_leader(id);
            }
            self.assign_role(id, role);
            ids.push(id);
        }
        Ok(ids)
    }
}
