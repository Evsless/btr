use crate::{console::cli::TrackerCli, error::BtrError};

pub struct CommandNode {
    cmd: String,
    description: String,
    children: Vec<CommandNode>,
    pub handler: Option<fn(&mut TrackerCli, &[&str]) -> Result<(), BtrError>>,
    extra_args: bool,
}

impl CommandNode {
    pub fn new(
        cmd: &str,
        description: &str,
        handler: Option<fn(&mut TrackerCli, &[&str]) -> Result<(), BtrError>>,
    ) -> Self {
        Self {
            cmd: cmd.to_string(),
            description: description.to_string(),
            children: Vec::new(),
            handler: handler,
            extra_args: handler.is_some(),
        }
    }

    pub fn add_child(mut self, child: CommandNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn find_command(&self, cmd: &[&str]) -> Option<&CommandNode> {
        /* Base case: the last CommandNode was reached. */
        if cmd.is_empty() {
            return Some(self);
        }

        /* Last node in a chain. Rest of arguments are optional and should not be handled. */
        if self.extra_args {
            return Some(self);
        }

        /* Recursive case: iterate until reaching the last subcommand */
        for child in &self.children {
            if child.cmd == cmd[0] {
                return child.find_command(&cmd[1..]);
            }
        }

        None
    }

    pub fn show_help(&self, context: &[&str]) {
        if context.is_empty() {
            println!(
                "? A budget tracker CLI application.\n\
                ?  Available commands:"
            );
        } else {
            println!("? HELP: {}", context.join(" "));
        }

        println!("?   {}", self.description);

        if !self.children.is_empty() {
            println!("? SUBCOMMANDS: ");
            for child in &self.children {
                println!("?   {} - {}", child.cmd, child.description);
            }
        }
    }
}
