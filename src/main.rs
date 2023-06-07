use std::io;
use std::io::Write;

pub mod meta_commands;
pub mod tokenizer;

fn main() -> io::Result<()> {

    println!(r#"Welcome to gql! a rusty version of sqlite
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
            let _ = match meta_commands::exec_meta_command(meta_commands::MetaCommand::from(user_input)) {
                Ok(res) => println!("{}", res),
                Err(err) => println!("Error: {}", err)
            };
        } else {
            // all other input we assume is SQL
            for token in tokenizer::tokenize(user_input) {
                println!("{}", token);
            }
        }
    }
}



// fn exec_sql_command(command: &str) {
//     // TO DO
// }