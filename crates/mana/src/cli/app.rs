use seahorse::{Command, Context, Flag, Action};

/// Custom implementation of `seahorse::App`
/// 
/// Currently, `seahorse::App` handles the _man_ pages by itself.
/// 
/// This implementation will allow adding a [docopt](http://docopt.org/)-compliant reporter.
#[derive(Default)]
pub struct App {
    /// Application name
    pub name: String,
    /// Application author
    pub author: Option<String>,
    /// Application description
    pub description: Option<String>,
    /// Application usage
    pub usage: Option<String>,
    /// Application version
    pub version: Option<String>,
    /// Application commands
    pub commands: Option<Vec<Command>>,
    /// Application action
    pub action: Option<Action>,
    /// Application flags
    pub flags: Option<Vec<Flag>>,
}

impl App {
    /// Create new instance of `App`
    ///
    /// Example
    ///
    /// ```
    /// use mana::App;
    ///
    /// let app = App::new("cli");
    /// ```
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Set author of the app
    ///
    /// Example
    ///
    /// ```
    /// use mana::App;
    ///
    /// let app = App::new("cli")
    ///     .author(env!("CARGO_PKG_AUTHORS"));
    /// ```
    pub fn author<T: Into<String>>(mut self, author: T) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set description of the app
    ///
    /// Example
    ///
    /// ```
    /// use mana::App;
    ///
    /// let app = App::new("cli")
    ///     .description(env!("CARGO_PKG_DESCRIPTION"));
    /// ```
    pub fn description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set usage of the app
    ///
    /// Example
    ///
    /// ```
    /// use mana::App;
    ///
    /// let app = App::new("cli");
    /// app.usage("cli [command] [arg]");
    /// ```
    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = Some(usage.into());
        self
    }

    /// Set version of the app
    ///
    /// Example
    ///
    /// ```
    /// use mana::App;
    ///
    /// let app = App::new("cli");
    /// app.version(env!("CARGO_PKG_VERSION"));
    /// ```
    pub fn version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set command of the app
    ///
    /// Example
    ///
    /// ```
    /// use mana::App;
    /// use seahorse::Command;
    ///
    /// let command = Command::new("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args));
    ///
    /// let app = App::new("cli")
    ///     .command(command);
    /// ```
    ///
    /// # Panics
    ///
    /// You cannot set a command named as same as registered ones.
    ///
    /// ```should_panic
    /// use mana::App;
    /// use seahorse::Command;
    ///
    /// let command1 = Command::new("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args));
    ///
    /// let command2 = Command::new("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args));
    ///
    /// let app = App::new("cli")
    ///     .command(command1)
    ///     .command(command2);
    /// ```
    pub fn command(mut self, command: Command) -> Self {
        if let Some(ref mut commands) = self.commands {
            if commands
                .iter()
                .any(|registered| registered.name == command.name)
            {
                panic!(format!(
                    r#"Command name "{}" is already registered."#,
                    command.name
                ));
            }
            (*commands).push(command);
        } else {
            self.commands = Some(vec![command]);
        }
        self
    }

    /// Set action of the app
    ///
    /// Example
    ///
    /// ```
    /// use mana::App;
    /// use seahorse::{Action, Context};
    ///
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// let app = App::new("cli")
    ///     .action(action);
    /// ```
    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    /// Set flag of the app
    ///
    /// Example
    ///
    /// ```
    /// use mana::App;
    /// use seahorse::{Flag, FlagType};
    ///
    /// let app = App::new("cli")
    ///     .flag(Flag::new("bool", FlagType::Bool))
    ///     .flag(Flag::new("int", FlagType::Int));
    /// ```
    pub fn flag(mut self, flag: Flag) -> Self {
        if let Some(ref mut flags) = self.flags {
            (*flags).push(flag);
        } else {
            self.flags = Some(vec![flag]);
        }
        self
    }

    /// Run app
    ///
    /// Example
    ///
    /// ```
    /// use std::env;
    /// use mana::App;
    ///
    /// let args: Vec<String> = env::args().collect();
    /// let app = App::new("cli");
    /// app.run(args);
    /// ```
    pub fn run(&self, args: Vec<String>) {

        // TODO: This is where we'll customize some stuff..
        if args.contains(&"--help".to_string()) {
            self.help();
            return;
        }

        let args = Self::normalized_args(args);
        let (cmd_v, args_v) = match args.len() {
            1 => args.split_at(1),
            _ => args[1..].split_at(1),
        };

        let cmd = match cmd_v.first() {
            Some(c) => c,
            None => {
                self.help();
                return;
            }
        };

        match self.select_command(&cmd) {
            Some(command) => command.run(args_v.to_vec(), self.generate_help_text()),
            None => match self.action {
                Some(action) => action(&Context::new(
                    args[1..].to_vec(),
                    self.flags.clone(),
                    self.generate_help_text(),
                )),
                None => self.help(),
            },
        }
    }

    /// Generate help text
    fn generate_help_text(&self) -> String {
        let mut text = String::new();

        text += &format!("Name\n\t{}\n\n", self.name);

        if let Some(author) = &self.author {
            text += &format!("Author:\n\t{}\n\n", author);
        }

        if let Some(description) = &self.description {
            text += &format!("Description:\n\t{}\n\n", description);
        }

        if let Some(usage) = &self.usage {
            text += &format!("Usage:\n\t{}\n", usage)
        }

        if let Some(flags) = &self.flags {
            for flag in flags {
                if let Some(usage) = &flag.usage {
                    text += &format!("\t{}\n", usage);
                }
            }
            text += "\n";
        }

        if let Some(commands) = &self.commands {
            text += "\nCommands:\n";

            let name_max_len = &commands.iter().map(|c| c.name.len()).max().unwrap();
            let whitespace = " ".repeat(name_max_len + 3);

            for c in commands {
                let command_name_len = c.name.len();

                let usage = match &c.usage {
                    Some(usage) => usage,
                    None => "",
                };

                text += &format!(
                    "\t{} {}: {}\n",
                    c.name,
                    " ".repeat(name_max_len - command_name_len),
                    usage
                );

                if let Some(flags) = &c.flags {
                    for flag in flags {
                        let usage = match &flag.usage {
                            Some(usage) => usage,
                            None => "",
                        };
                        text += &format!("\t{}{}\n", whitespace, usage);
                    }
                }

                text += "\n";
            }
        }

        if let Some(version) = &self.version {
            text += &format!("Version:\n\t{}\n", version);
        }

        text
    }

    /// Application help
    /// Displays information about the application
    fn help(&self) {
        println!("{}", self.generate_help_text());
    }

    /// Select command
    /// Gets the Command that matches the string passed in the argument
    fn select_command(&self, cmd: &str) -> Option<&Command> {
        match &self.commands {
            Some(commands) => commands.iter().find(|command| match &command.alias {
                Some(alias) => command.name == cmd || alias.iter().any(|a| a == cmd),
                None => command.name == cmd,
            }),
            None => None,
        }
    }

    /// Split arg with "=" to unify arg notations.
    /// --flag=value => ["--flag", "value"]
    /// --flag value => ["--flag", "value"]
    fn normalized_args(raw_args: Vec<String>) -> Vec<String> {
        raw_args.iter().fold(Vec::<String>::new(), |mut acc, cur| {
            if cur.starts_with('-') && cur.contains('=') {
                let mut splitted_flag: Vec<String> =
                    cur.splitn(2, '=').map(|s| s.to_owned()).collect();
                acc.append(&mut splitted_flag);
            } else {
                acc.push(cur.to_owned());
            }
            acc
        })
    }
}