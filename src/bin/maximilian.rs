use filler::run;
use filler::strategy::attack::Attack;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run::run(Attack)?;
    Ok(())
}
