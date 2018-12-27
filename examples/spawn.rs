use rs_xfoil::XfoilRunner;

fn main() {
    let result= XfoilRunner::new("/usr/local/bin/xfoil")
        .naca("2414")
        .reynolds(100_000)
        .polar_accumulation("foo")
        .angle_of_attack(4.0)
        .dispatch().unwrap();
}