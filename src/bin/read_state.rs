use config_layer::AppConfig;
use state_crystal::Crystalizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = AppConfig::load("config.toml")?;
    let loaded: (u64, usize, f64, f64) = Crystalizer::thaw(&cfg.state_file)?;
    println!("tick: {}", loaded.0);
    println!("agents: {}", loaded.1);
    println!("gain: {}", loaded.2);
    println!("resistance: {}", loaded.3);
    Ok(())
}
