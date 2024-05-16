use crate::{Action, Help};
use std::error::Error;

/// Application command type
#[derive(Default)]
pub struct Command {
    /// Command name
    pub name: String,
    /// Command alias
    pub alias: Option<Vec<String>>,
    /// Command description
    pub description: Option<String>,
    /// Command usage
    pub usage: Option<String>,
    /// Command action
    pub action: Option<Action>,
}

impl Command {
    /// Create new instance of `Command`
    ///
    /// Example
    ///
    /// ```
    /// use suihorse::Command;
    ///
    /// let command = Command::new("cmd");
    /// ```
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Set description of the command
    ///
    /// Example
    ///
    /// ```
    /// use suihorse::Command;
    ///
    /// let command = Command::new("cmd")
    ///     .description("cli sub command");
    /// ```
    pub fn description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set usage of the command
    ///
    /// Example
    ///
    /// ```
    /// use suihorse::Command;
    ///
    /// let command = Command::new("cmd")
    ///     .usage("cli cmd [arg]");
    /// ```
    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = Some(usage.into());
        self
    }

    /// Set action of the command
    ///
    /// Example
    ///
    /// ```
    /// use suihorse::{Command, Context, Action};
    ///
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// let command = Command::new("cmd")
    ///     .action(action);
    /// ```
    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    /// Set alias of the command
    ///
    /// Example
    ///
    /// ```
    /// use suihorse::Command;
    ///
    /// let command = Command::new("cmd")
    ///     .alias("c");
    /// ```
    pub fn alias<T: Into<String>>(mut self, name: T) -> Self {
        if let Some(ref mut alias) = self.alias {
            (*alias).push(name.into());
        } else {
            self.alias = Some(vec![name.into()]);
        }
        self
    }

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

    /// Call this function only from `App`
    pub fn run_with_result(&self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let args = Self::normalized_args(args);

        match self.action {
            Some(action) => {
                if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
                    self.help();
                    return Ok(());
                }
                action(args.to_vec());
                return Ok(());
            },
            None => {
                self.help();
                return Ok(());
            }
        }
    }
}

impl Help for Command {
    fn help_text(&self) -> String {
        let mut text = String::new();

        if let Some(description) = &self.description {
            text += &format!("Description:\n\t{}\n\n", description);
        }

        if let Some(usage) = &self.usage {
            text += &format!("Usage:\n\t{}\n\n", usage);
        }

        text
    }
}
