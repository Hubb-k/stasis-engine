use agent_framework::{Agent, AgentStatus, Environment, PopulationManager};
use config_layer::AppConfig;
use resonance_core::calculate_interference;
use serde::{Deserialize, Serialize};
use state_crystal::Crystalizer;

#[derive(Serialize, Deserialize, Clone)]
struct SimpleAgent {
    id: String,
    energy: f64,
    age: u32,
}

impl SimpleAgent {
    fn new(id: &str, energy: f64) -> Self {
        Self {
            id: id.to_string(),
            energy,
            age: 0,
        }
    }
}

impl Agent for SimpleAgent {
    fn update(&mut self, env: &Environment) -> AgentStatus {
        self.age += 1;

        let interference = calculate_interference(self.energy, env.gain);
        self.energy += env.gain - env.resistance + interference * env.interference_factor;

        if self.energy < 0.0 {
            AgentStatus::Expired
        } else if self.energy > env.mutation_threshold && self.age > env.min_age_for_mutation {
            let mut mutated = self.clone();
            mutated.energy = env.mutation_reset_energy;
            mutated.age = 0;
            mutated.id = format!("{}_mutated", self.id);
            AgentStatus::Mutate(Box::new(mutated))
        } else {
            AgentStatus::Active
        }
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = AppConfig::load("config.toml")?;

    let env = Environment {
        gain: cfg.gain,
        resistance: cfg.resistance,
        mutation_threshold: cfg.mutation_threshold,
        min_age_for_mutation: cfg.min_age_for_mutation,
        mutation_reset_energy: cfg.mutation_reset_energy,
        interference_factor: cfg.interference_factor,
    };

    let mut manager = PopulationManager::new();

    for i in 0..cfg.initial_agents_count {
        let agent = SimpleAgent::new(&format!("agent_{}", i), cfg.initial_energy);
        manager.add_agent(Box::new(agent));
    }

    let mut tick = 0;
    while manager.agent_count() > 0 && tick < cfg.max_ticks {
        manager.tick(&env);
        tick += 1;
    }

    let state = (tick, manager.agent_count(), env.gain, env.resistance);
    Crystalizer::freeze(&cfg.state_file, &state)?;

    Ok(())
}
