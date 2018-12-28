# rs-xfoil

This crate provides a Rust interface to Xfoil, an aerodynamics simulation program written by
Mark Drela in Fortran. This will allow users to perform calculations using Xfoil, while seamlessly
using the results in Rust code.

## Model

The crate works by building up a command sequence in an `XfoilRunner`. Xfoil can be configured fully
before actually running the process. Valid configurational states are tracked internally. After
configuring the runner, the process can be dispatched, which consumes the runner and returns the
result of the calculation.

## Example

```rust
extern crate rs_xfoil;

fn main() {
    let result = rs_xfoil::Config::new("/usr/local/bin/xfoil")
        .naca("2414")
        .reynolds(100_000)
        .polar_accumulation("test_run")
        .angle_of_attack(4.0)
        .get_runner()
        .unwrap()
        .dispatch()
        .unwrap();
    println!("{:?}", result);
}
```
