use super::{Delegation, Level};
use z_core::{Agent, AgentId};
use z_runtime::{Runtime, supervisor::RestartPolicy};
use crate::PatternError;
use std::collections::HashMap;

/// Hierarchical organization structure
#[derive(Debug, Clone)]
pub struct Hierarchy {
    name: String,
    levels: Vec<Level>,
    agents: HashMap<AgentId, Level>,
    delegations: Vec<Delegation>,
}

impl Hierarchy {
    /// Create a new hierarchy
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            levels: Vec::new(),
            agents: HashMap::new(),
            delegations: Vec::new(),
        }
    }

    /// Add a level
    pub fn add_level(&mut self, level: Level) {
        self.levels.push(level);
    }

    /// Assign agent to level
    pub fn assign_agent(&mut self, agent_id: AgentId, level: Level) {
        self.agents.insert(agent_id, level);
    }

    /// Get agent's level
    pub fn get_level(&self, agent_id: &AgentId) -> Option<&Level> {
        self.agents.get(agent_id)
    }

    /// Delegate task
    pub fn delegate(&mut self, delegation: Delegation) {
        self.delegations.push(delegation);
    }

    /// Get all delegations
    pub fn delegations(&self) -> &[Delegation] {
        &self.delegations
    }

    /// Get hierarchy name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all levels
    pub fn levels(&self) -> &[Level] {
        &self.levels
    }

    /// Spawn agents into `runtime`, assign each to its level, and return their IDs.
    /// Each tuple is `(agent, name, level)`.
    pub async fn spawn_agents(
        &mut self,
        runtime: &Runtime,
        agents: Vec<(Box<dyn Agent>, String, Level)>,
        policy: RestartPolicy,
    ) -> Result<Vec<AgentId>, PatternError> {
        let mut ids = Vec::with_capacity(agents.len());
        for (agent, name, level) in agents {
            let id = runtime
                .spawn_with_policy(agent, name, policy.clone())
                .await
                .map_err(|e| PatternError::SpawnFailed(e.to_string()))?;
            self.assign_agent(id, level);
            ids.push(id);
        }
        Ok(ids)
    }
}
