mod app;

//use seahorse::{Command, Context, Flag, FlagType, Action};
use std::env;
pub use app::App;

/// Setup the Mana CLI, retrieve and handle inputs
pub fn start_cli() {
    let args: Vec<String> = env::args().collect(); 
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("mana [command] [--option] [<argument>]")
        .action(|c| println!("Hello, {:?}", c.args))
        .run(args)
}