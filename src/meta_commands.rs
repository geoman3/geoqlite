pub enum MetaCommand {
    Exit(Vec<String>),
    Help(Vec<String>),
    Open(Vec<String>),
    Save(Vec<String>),
    Tables(Vec<String>),
    Unknown(Vec<String>)
}

// matches strings input by the user with values in the enum MetaCommand
impl MetaCommand {
    pub fn from(user_input: &str) -> MetaCommand {
        let words: Vec<String> = user_input.split(" ").map(|s| s.to_string()).collect();
        let command_str: &str = words[0].as_str();
        let args: Vec<String> = words[1..].to_vec();
        match command_str {
            ".exit" => MetaCommand::Exit(args),
            ".help" => MetaCommand::Help(args),
            ".open" => MetaCommand::Open(args),
            ".save" => MetaCommand::Save(args),
            ".tables" => MetaCommand::Tables(args),
            _ => MetaCommand::Unknown(args)
        }
    }
}

pub fn exec_meta_command(command: MetaCommand) -> Result<&'static str, &'static str> {
    match command {
        MetaCommand::Exit(_) => {
            std::process::exit(0)
        },
        MetaCommand::Help(_) => {
            let message = r#"
available commands:
.exit            => quits the program
.help            => displays this message
.open <FILENAME> => opens a new database from FILENAME
.save <FILENAME> => saves current in memory database to FILENAME
.tables          => lists available tables
            "#;
            Ok(message)
        },
        MetaCommand::Open(_) => {
            Err("Not Implemented")
        },
        MetaCommand::Save(_) => {
            Err("Not Implemented")
        },
        MetaCommand::Tables(_) => {
            Err("Not Implemented")
        },
        MetaCommand::Unknown(_) => {
            Err("Command not recognised")
        }
    }
}
