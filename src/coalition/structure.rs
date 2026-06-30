use super::Strategy;
use z_core::{Agent, AgentId};
use z_runtime::{Runtime, supervisor::RestartPolicy};
use crate::PatternError;
use std::collections::HashSet;

/// Coalition of agents
#[derive(Debug, Clone)]
pub struct Coalition {
    name: String,
    members: HashSet<AgentId>,
    strategy: Option<Strategy>,
    value: f64,
}

impl Coalition {
    /// Create a new coalition
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: HashSet::new(),
            strategy: None,
            value: 0.0,
        }
    }

    /// Add member
    pub fn add_member(&mut self, agent_id: AgentId) -> bool {
        self.members.insert(agent_id)
    }

    /// Remove member
    pub fn remove_member(&mut self, agent_id: &AgentId) -> bool {
        self.members.remove(agent_id)
    }

    /// Check if agent is member
    pub fn has_member(&self, agent_id: &AgentId) -> bool {
        self.members.contains(agent_id)
    }

    /// Set strategy
    pub fn set_strategy(&mut self, strategy: Strategy) {
        self.strategy = Some(strategy);
    }

    /// Set coalition value
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get members
    pub fn members(&self) -> impl Iterator<Item = &AgentId> {
        self.members.iter()
    }

    /// Get size
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Get strategy
    pub fn strategy(&self) -> Option<&Strategy> {
        self.strategy.as_ref()
    }

    /// Get value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Spawn agents into `runtime`, enroll them as coalition members, and return their IDs.
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
