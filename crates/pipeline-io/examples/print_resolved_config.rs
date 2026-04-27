use pipeline_core::RepoPaths;
use pipeline_io::load_process_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = RepoPaths::from_current_dir()?;
    let config = load_process_config(&repo)?;
    println!("{}", config.sanitized_json());
    Ok(())
}
