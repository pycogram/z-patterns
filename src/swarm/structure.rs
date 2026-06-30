use super::Behavior;
use z_core::{Agent, AgentId};
use z_runtime::{Runtime, supervisor::RestartPolicy};
use crate::PatternError;

/// Swarm of agents
#[derive(Debug, Clone)]
pub struct Swarm {
    name: String,
    members: Vec<AgentId>,
    behavior: Option<Behavior>,
}

impl Swarm {
    /// Create a new swarm
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: Vec::new(),
            behavior: None,
        }
    }

    /// Add member
    pub fn add_member(&mut self, agent_id: AgentId) {
        self.members.push(agent_id);
    }

    /// Set behavior
    pub fn set_behavior(&mut self, behavior: Behavior) {
        self.behavior = Some(behavior);
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get members
    pub fn members(&self) -> &[AgentId] {
        &self.members
    }

    /// Get behavior
    pub fn behavior(&self) -> Option<&Behavior> {
        self.behavior.as_ref()
    }

    /// Get size
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Spawn a set of agents into `runtime`, register them as swarm members,
    /// and return the list of assigned `AgentId`s.
    pub async fn spawn_agents(
        &mut self,
        runtime: &Runtime,
        agents: Vec<(Box<dyn Agent>, String)>,
        policy: RestartPolicy,
    ) -> Result<Vec<AgentId>, PatternError> {
        let mut ids = Vec::with_capacity(agents.len());
        for (agent, name) in agents {
            let id = runtime
                .spawn_with_policy(agent, name, policy.clone())
                .await
                .map_err(|e| PatternError::SpawnFailed(e.to_string()))?;
            self.add_member(id);
            ids.push(id);
        }
        Ok(ids)
    }
}
