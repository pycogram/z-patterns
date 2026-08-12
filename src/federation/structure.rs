use super::Policy;
use agent_core::{Agent, AgentId};
use runtime::{Runtime, supervisor::RestartPolicy};
use crate::PatternError;
use std::collections::{HashMap, HashSet};

/// Federation of autonomous agents
#[derive(Debug, Clone)]
pub struct Federation {
    name: String,
    members: HashSet<AgentId>,
    policies: HashMap<String, Policy>,
    weights: HashMap<AgentId, f64>,
}

impl Federation {
    /// Create a new federation
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: HashSet::new(),
            policies: HashMap::new(),
            weights: HashMap::new(),
        }
    }

    /// Add member
    pub fn add_member(&mut self, agent_id: AgentId) -> bool {
        let added = self.members.insert(agent_id); // ← FIXED: Removed .clone()
        if added {
            self.weights.insert(agent_id, 1.0);
        }
        added
    }

    /// Remove member
    pub fn remove_member(&mut self, agent_id: &AgentId) -> bool {
        self.weights.remove(agent_id);
        self.members.remove(agent_id)
    }

    /// Set member weight (for weighted voting)
    pub fn set_weight(&mut self, agent_id: AgentId, weight: f64) {
        if self.members.contains(&agent_id) {
            self.weights.insert(agent_id, weight.max(0.0));
        }
    }

    /// Add policy
    pub fn add_policy(&mut self, policy: Policy) {
        self.policies.insert(policy.name().to_string(), policy);
    }

    /// Get policy
    pub fn get_policy(&self, name: &str) -> Option<&Policy> {
        self.policies.get(name)
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get members
    pub fn members(&self) -> impl Iterator<Item = &AgentId> {
        self.members.iter()
    }

    /// Get member weight
    pub fn weight(&self, agent_id: &AgentId) -> Option<f64> {
        self.weights.get(agent_id).copied()
    }

    /// Get size
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Get all policies
    pub fn policies(&self) -> impl Iterator<Item = &Policy> {
        self.policies.values()
    }

    /// Spawn agents into `runtime`, enroll them as federation members, and return their IDs.
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
