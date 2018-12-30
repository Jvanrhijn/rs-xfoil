use rs_xfoil::{Config, error::XfoilError};

fn main() -> Result<(), XfoilError> {
    let result= Config::new("/usr/local/bin/xfoil")
        .naca("2414")
        .reynolds(100_000)
        .pacc_random()
        .angle_of_attack(4.0)
        .get_runner()?
        .dispatch()?;
    println!("{:?}", result);
    Ok(())
}