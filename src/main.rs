use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

trait Command {
    fn execute(&self) -> Result<(), &str>;
}

struct NullCmd {}
impl NullCmd {
    fn new() -> Self {
        NullCmd {}
    }
}
impl Command for NullCmd {
    fn execute(&self) -> Result<(), &str> {
        Err("Unknown command")
    }
}

struct PingCmd {
    args: Option<String>,
}
impl PingCmd {
    fn new(args: Option<String>) -> Self {
        PingCmd { args }
    }
}
impl Command for PingCmd {
    fn execute(&self) -> Result<(), &str> {
        // TODO:
        Err("Unknown command")
    }
}

enum Cmd {
    Ping,
    Null,
}
struct Command2 {
    raw_commands: HashMap<String, Cmd>,
}
impl Command2 {
    fn new() -> Self {
        Command2 {
            raw_commands: HashMap::from([("ping".to_string(), Cmd::Ping)]),
        }
    }

    fn one_by(&self, a_cmd_name: &str, args: Option<String>) -> Box<dyn Command> {
        match self.raw_commands.get(a_cmd_name).unwrap_or(&Cmd::Null) {
            Cmd::Ping => Box::new(PingCmd::new(args)),
            Cmd::Null => Box::new(NullCmd::new()),
        }
    }
}

struct Commands {
    raw_commands: HashMap<String, Box<dyn Command>>,
    null_cmd: Box<dyn Command>,
}
impl Commands {
    fn new(raw_commands: HashMap<String, Box<dyn Command>>) -> Self {
        Commands {
            raw_commands,
            null_cmd: Commands::default_cmd(),
        }
    }

    fn one_by(&self, a_cmd_name: &str) -> &Box<dyn Command> {
        self.raw_commands.get(a_cmd_name).unwrap_or(&self.null_cmd)
    }

    fn default_cmd() -> Box<dyn Command> {
        Box::new(NullCmd::new())
    }
}
impl Default for Commands {
    fn default() -> Self {
        let mut raw_commands: HashMap<String, Box<dyn Command>> = HashMap::new();
        raw_commands.insert(
            "ping".to_string(),
            Box::new(PingCmd::new(Some("--a".to_string()))),
        );
        raw_commands.insert("null".to_string(), Box::new(NullCmd::new()));
        Commands {
            raw_commands,
            null_cmd: Self::default_cmd(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw_commands() -> HashMap<String, Box<dyn Command>> {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
        commands.insert(
            "ping".to_string(),
            Box::new(PingCmd::new(Some("--a".to_string()))),
        );
        commands.insert("other".to_string(), Box::new(NullCmd::new()));
        commands
    }

    fn execute_command<T: Command>(cmd: &T) -> Result<(), &str> {
        cmd.execute()
    }

    #[test]
    fn command2_new() {
        let command2 = Command2::new();

        assert!(command2
            .one_by("ping", Some("".to_string()))
            .execute()
            .is_err());
    }

    #[test]
    fn commands_new() {
        let commands = Commands::new(raw_commands());

        assert!(commands.one_by("ping").execute().is_err());
    }

    #[test]
    fn commands_default() {
        let commands = Commands::default();

        assert!(commands.one_by("ping").execute().is_err());
    }

    #[test]
    fn commands_unknown_cmd() {
        let commands = Commands::new(raw_commands());

        assert!(commands.one_by("asdf").execute().is_err());
    }

    #[test]
    fn command_new() {
        let ping_cmd = PingCmd::new(Some("--a".to_string()));

        assert!(execute_command(&ping_cmd).is_err());
    }

    #[test]
    fn lerning_test_command_map() {
        let mut command_map: HashMap<String, Box<dyn Command>> = HashMap::new();

        // Insert commands into the map
        let ping_command = Box::new(PingCmd::new(Some("--a".to_string())));
        let other_command = Box::new(NullCmd::new());

        command_map.insert("ping".to_string(), ping_command);
        command_map.insert("other".to_string(), other_command);

        assert!(command_map.get("ping").unwrap().execute().is_err());
        assert!(command_map.get("other").unwrap().execute().is_err());
    }
}
