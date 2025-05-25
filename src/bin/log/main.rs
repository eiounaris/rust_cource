use log::info;
use env_logger::{Builder, Target};

fn main() {
    // env_logger::init();

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();


    info!("starting up");

    // ...
}
