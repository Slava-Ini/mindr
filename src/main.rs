use mindr;

// TODO: think if it's good to add other crate (not mindr) kind of like namespace for config
fn main() {
    let config = mindr::Config::init();
    println!("config: {:#?}", config.unwrap());
}

// --- Future ---
// TODO: add option to pass path as args to program and then to `init()`
