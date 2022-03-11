fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    dbg!(args);

    panic!("Exiting with failure to show output");
}
