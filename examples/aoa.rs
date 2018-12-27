extern crate rand;
use rand::{Rng};
use rand::distributions::Alphanumeric;
use std::collections::HashMap;
use std::vec::Vec;
use rs_xfoil::{XfoilRunner, XfoilError};

fn main() -> Result<(), XfoilError> {
    let result= XfoilRunner::new("/usr/local/bin/xfoil")
        .naca("2414")
        .reynolds(100_000)
        .polar_accumulation(&format!("/tmp/{}",
           rand::thread_rng()
               .sample_iter(&Alphanumeric)
               .take(10)
               .collect::<String>())
        )
        .angle_of_attack(4.0)
        .dispatch()?;
    println!("{:?}", result);
    Ok(())
}