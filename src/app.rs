use crate::{Command, Help};
use std::error::Error;

/// Command and application action type
///
/// Example
///
/// ```
/// use suihorse::{Action, Context};
///
/// let action: Action = |c: &Context| {
///     println!("{:?}", c.args);
/// };
/// ```
pub type Action = fn(Vec<String>);

/// Multiple action application entry point
pub struct App {
    /// usage: "cli [command] [arg]"
    pub usage: String,
    /// Application commands including default cmds and dev defined
    pub commands: Vec<Command>,
    /// default action displaying recent data and config
    pub action: Action,
}
// TODO add default action and commands 
impl Default for App {
    fn default() -> Self {
        Self {
            usage: "cli [command] [arg]".to_string(),
            commands: vec![],
            action: |_| { println!("j") },
        }
    }
}

impl App {
    /// Create new instance of `App`
    ///
    /// Example
    ///
    /// ```
    /// use suihorse::App;
    ///
    /// let app = App::new("cli");
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set usage of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::App;
    ///
    /// let app = App::new("cli");
    /// app.usage("cli [command] [arg]");
    /// ```
    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = usage.into();
        self
    }

    /// Set command of the app
    ///
    /// Example
    ///
    /// ```
    /// use suihorse::{App, Command};
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
    /// use suihorse::{App, Command};
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
        if self.commands
            .iter()
            .any(|registered| registered.name == command.name)
        {
            panic!(r#"Command name "{}" is already registered."#, command.name);
        }
        self.commands.push(command);
        self
    }

    /// Set action of the app
    ///
    /// Example
    ///
    /// ```
    /// use suihorse::{Action, App, Context};
    ///
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// let app = App::new("cli")
    ///     .action(action);
    /// ```
    pub fn action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }

    /// Run app
    ///
    /// Example
    ///
    /// ```
    /// use std::env;
    /// use suihorse::App;
    ///
    /// let args: Vec<String> = env::args().collect();
    /// let app = App::new("cli");
    /// app.run(args);
    /// ```
    pub fn run(&self, args: Vec<String>) {
        match self.run_with_result(args) {
            Ok(_) => return,
            Err(e) => panic!("{}", e),
        }
    }

    /// Run app, returning a result
    ///
    /// Example
    ///
    /// ```
    /// use std::env;
    /// use suihorse::App;
    ///
    /// let args: Vec<String> = env::args().collect();
    /// let app = App::new("cli");
    /// let result = app.run_with_result(args);
    /// ```
    pub fn run_with_result(&self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let args = Self::normalized_args(args);
        let (cmd_v, args_v) = args.split_at(1);
        let cmd = cmd_v.first().unwrap();
        
        // gets the command in the App that matches `cmd` or return None
        let command = self.commands.iter().find(|command| match &command.alias {
            Some(alias) => &command.name == cmd || alias.iter().any(|a| a == cmd),
            None => &command.name == cmd,
        });

        match command {
            // if there is a command we run it
            Some(command) => return command.run_with_result(args_v.to_vec()),
            // if the 2nd arg is not a command we run App action
            None => {
                // except if there's a help flag
                if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
                    self.help();
                    return Ok(());
                };
                let action = self.action;
                action(args[1..].to_vec());
                return Ok(());
            }
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

    fn command_help_text(&self) -> String {
        let mut text = String::new();

        text += "\nCommands:\n";

        let name_max_len = &self.commands
            .iter()
            .map(|c| {
                if let Some(alias) = &c.alias {
                    format!("{}, {}", alias.join(", "), c.name).len()
                } else {
                    c.name.len()
                }
            })
            .max()
            .unwrap();

        for c in self.commands.iter() {
            let command_name = if let Some(alias) = &c.alias {
                format!("{}, {}", alias.join(", "), c.name)
            } else {
                c.name.clone()
            };

            let description = match &c.description {
                Some(description) => description,
                None => "",
            };

            text += &format!(
                "\t{} {}: {}\n",
                command_name,
                " ".repeat(name_max_len - command_name.len()),
                description
            );
        }

        text
    }
}

impl Help for App {
    fn help_text(&self) -> String {
        let mut text = String::new();
        text += &format!("Usage:\n\t{}\n\n", self.usage);
        text += &self.command_help_text();

        text
    }
}