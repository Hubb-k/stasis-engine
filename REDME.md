
# Stasis Engine

![CI](https://github.com/Hubb-k/stasis-engine/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-2021-orange.svg)

A framework for agent-based modeling with support for wave physics, evolutionary algorithms, and binary state serialization.


## Project structure

```text
stasis-engine/
|
|-- Cargo.toml
|-- config.toml
|-- README.md
|-- LICENSE
|-- CHANGELOG.md
|-- CONTRIBUTING.md
|-- .gitignore
|-- .github/
|   |-- workflows/
|       |-- ci.yml
|
|-- src/
|   |-- main.rs
|   |-- bin/
|       |-- read_state.rs
|
|-- agent-framework/
|   |-- Cargo.toml
|   |-- src/
|       |-- lib.rs
|
|-- config-layer/
|   |-- Cargo.toml
|   |-- src/
|       |-- lib.rs
|
|-- resonance-core/
|   |-- Cargo.toml
|   |-- src/
|       |-- lib.rs
|
|-- state-crystal/
    |-- Cargo.toml
    |-- src/
        |-- lib.rs
```


## Configuration

All simulation parameters are defined in `config.toml`:

```toml
# Environment parameters
gain = 5.0                    # Energy gain per tick
resistance = 2.0              # Energy loss per tick

# Initial state
initial_agents_count = 10     # Number of agents at start
initial_energy = 50.0         # Starting energy for each agent

# Simulation control
max_ticks = 100               # Maximum simulation duration
state_file = "state.bin"      # Output file for state snapshot

# Mutation parameters
mutation_threshold = 100.0    # Energy level to trigger mutation
min_age_for_mutation = 10     # Minimum age before mutation possible
mutation_reset_energy = 50.0  # Energy after mutation
interference_factor = 0.1     # Wave interference impact
```


## Usage

### Run simulation

```bash
cargo run --release
```

The simulation will:
1. Load configuration from config.toml
2. Create initial agents with specified parameters
3. Run ticks until max_ticks is reached or all agents expire
4. Save final state to the configured state_file

### Read saved state

```bash
cargo run --release --bin read_state
```


## Business applications

This architecture is designed for complex, non-linear environments where wave effects, adaptation, and high-performance computation are critical.


### 1. FinTech and Algorithmic Trading

- Agents: Trading bots, market makers, retail traders.
- Environment: gain represents market volatility or trend strength; resistance represents liquidity, fees, or slippage.
- Resonance: Models algorithmic resonance (e.g., flash crashes caused by multiple bots triggering simultaneously). Interference calculations map buying/selling pressure waves.
- Mutation: Adaptive strategy switching (e.g., from mean-reversion to trend-following).
- Value: Portfolio stress testing, backtesting in realistic noisy environments, risk management.


### 2. Telecommunications and Network Design (5G/6G, IoT)

- Agents: Data packets, IoT devices, base stations.
- Environment: gain is signal strength or channel bandwidth; resistance is signal attenuation, latency, or noise.
- Resonance: Direct calculation of radio signal interference (constructive/destructive) to optimize antenna placement or frequency allocation.
- Mutation: Dynamic routing protocol changes or frequency hopping in response to jamming or congestion.
- Value: Network capacity planning, Smart City simulation, pre-deployment optimization.


### 3. Energy and Smart Grids

- Agents: Consumers, generators, energy storage systems (batteries).
- Environment: gain is peak generation or demand; resistance is transmission line loss.
- Resonance: Models harmonic distortions and cascade failures in power grids (e.g., one line failure causing resonant load spikes on others).
- Mutation: Dynamic load shedding, switching power sources.
- Value: Blackout prevention, renewable energy integration, energy trading optimization.


### 4. Marketing, PR, and Sociodynamics

- Agents: Consumers, influencers, opinion leaders.
- Environment: gain is campaign intensity or content virality; resistance is banner blindness, skepticism, or information noise.
- Resonance: Models echo chambers. When marketing messages are "in phase", constructive interference occurs (viral growth). "Out of phase" messages cancel each other out (campaign cannibalization).
- Mutation: Shifts in consumer preferences or migration between social groups.
- Value: Media mix optimization, viral campaign prediction, reputation management.


### 5. Logistics and Supply Chain (Digital Twins)

- Agents: Cargo, vehicles, warehouses.
- Environment: gain is product demand; resistance is road capacity or customs delays.
- Resonance: Models the Bullwhip Effect, where small retail demand fluctuations cause resonant, amplified oscillations at the manufacturing level.
- Mutation: Dynamic rerouting, supplier switching.
- Value: Supply chain digital twins, bottleneck identification, inventory optimization.


## Technical advantages

- Deterministic and reproducible: bincode serialization and strict typing guarantee bit-identical results across different machines given the same random seeds.
- Auditability: The state-crystal module allows freezing the system state at any tick for post-mortem analysis, which is critical in regulated industries.
- High performance: The workspace structure and Send + Sync trait bounds lay the foundation for easy parallelization (e.g., via rayon) or GPU offloading in the future.
