pub trait Agent: Send + Sync {
    fn update(&mut self, env: &Environment) -> AgentStatus;
    fn get_id(&self) -> String;
}

pub struct Environment {
    pub gain: f64,
    pub resistance: f64,
    pub mutation_threshold: f64,
    pub min_age_for_mutation: u32,
    pub mutation_reset_energy: f64,
    pub interference_factor: f64,
}

pub enum AgentStatus {
    Active,
    Expired,
    Mutate(Box<dyn Agent>),
}

#[derive(Default)]
pub struct PopulationManager {
    agents: Vec<Box<dyn Agent>>,
}

impl PopulationManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_agent(&mut self, agent: Box<dyn Agent>) {
        self.agents.push(agent);
    }

    pub fn tick(&mut self, env: &Environment) {
        let mut new_agents = Vec::new();

        self.agents.retain_mut(|agent| match agent.update(env) {
            AgentStatus::Active => true,
            AgentStatus::Expired => false,
            AgentStatus::Mutate(mutated) => {
                new_agents.push(mutated);
                false
            }
        });

        self.agents.append(&mut new_agents);
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestAgent {
        id: String,
        ticks: u32,
        behavior: String,
    }

    impl TestAgent {
        fn new(id: &str, behavior: &str) -> Self {
            Self {
                id: id.to_string(),
                ticks: 0,
                behavior: behavior.to_string(),
            }
        }
    }

    impl Agent for TestAgent {
        fn update(&mut self, _env: &Environment) -> AgentStatus {
            self.ticks += 1;
            match self.behavior.as_str() {
                "active" => AgentStatus::Active,
                "expired" => AgentStatus::Expired,
                "mutate" if self.ticks >= 2 => {
                    let mut new = TestAgent::new(&format!("{}_mut", self.id), "active");
                    new.ticks = 0;
                    AgentStatus::Mutate(Box::new(new))
                }
                _ => AgentStatus::Active,
            }
        }

        fn get_id(&self) -> String {
            self.id.clone()
        }
    }

    fn test_env() -> Environment {
        Environment {
            gain: 1.0,
            resistance: 1.0,
            mutation_threshold: 100.0,
            min_age_for_mutation: 1,
            mutation_reset_energy: 50.0,
            interference_factor: 0.1,
        }
    }

    #[test]
    fn test_active_agent_survives() {
        let mut manager = PopulationManager::new();
        manager.add_agent(Box::new(TestAgent::new("a1", "active")));
        let env = test_env();

        manager.tick(&env);
        manager.tick(&env);

        assert_eq!(manager.agent_count(), 1);
    }

    #[test]
    fn test_expired_agent_removed() {
        let mut manager = PopulationManager::new();
        manager.add_agent(Box::new(TestAgent::new("a1", "expired")));
        let env = test_env();

        manager.tick(&env);

        assert_eq!(manager.agent_count(), 0);
    }

    #[test]
    fn test_mutation_replaces_agent() {
        let mut manager = PopulationManager::new();
        manager.add_agent(Box::new(TestAgent::new("a1", "mutate")));
        let env = test_env();

        manager.tick(&env);
        assert_eq!(manager.agent_count(), 1);

        manager.tick(&env);
        assert_eq!(manager.agent_count(), 1);

        manager.tick(&env);
        assert_eq!(manager.agent_count(), 1);
    }

    #[test]
    fn test_multiple_agents() {
        let mut manager = PopulationManager::new();
        manager.add_agent(Box::new(TestAgent::new("a1", "active")));
        manager.add_agent(Box::new(TestAgent::new("a2", "active")));
        manager.add_agent(Box::new(TestAgent::new("a3", "expired")));
        let env = test_env();

        manager.tick(&env);

        assert_eq!(manager.agent_count(), 2);
    }
}
