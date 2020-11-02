mod app;

pub use app::App;
use seahorse::{Command, Context};
use std::env;

fn sub_action(_c: &Context) {
    println!("{}", 1);
}

/// Setup the Mana CLI, retrieve and handle inputs
pub fn start_cli() {
    let args: Vec<String> = env::args().collect();
    let _f = |c: &Context| println!("{:?}", c.args);
    let command = Command::new("hello")
        .usage("cli hello [arg]")
        .action(|c| sub_action(&c));

    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("mana [command] [--option] [<argument>]")
        .action(|c| println!("{:?}", c.args))
        .command(command)
        .run(args)
}
