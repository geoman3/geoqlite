use std::io;
use std::io::Write;

fn main() -> io::Result<()> {

    println!(r#"Welcome to geoql! a rusty version of sqlite,
    type .help to see a list of commands"#);

    loop {
        // Basic REPL loop
        print!("gql >>");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        // strip EOL chars (accounting for windows or unix machines)
        let user_input: &str = buffer
            .strip_suffix("\r\n")
            .or(buffer.strip_suffix("\n"))
            .unwrap_or(&buffer);

        if user_input.starts_with(".") {
            let _ = match exec_meta_command(MetaCommand::from(user_input)) {
                Ok(res) => println!("{}", res),
                Err(err) => println!("Error: {}", err)
            };
        } else {
            // exec_sql_command(command)
        }
    }
}

fn exec_meta_command(command: MetaCommand) -> Result<&'static str, &'static str> {
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

enum MetaCommand {
    Exit(Vec<String>),
    Help(Vec<String>),
    Open(Vec<String>),
    Save(Vec<String>),
    Tables(Vec<String>),
    Unknown(Vec<String>)
}

impl MetaCommand {
    fn from(user_input: &str) -> MetaCommand {
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

// fn exec_sql_command(command: &str) {
//     // TO DO
// }