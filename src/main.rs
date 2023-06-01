use std::io::{self, Write};

fn main() -> io::Result<()> {
    
    loop {
        print!("db > ");
        io::stdout().flush().expect("Unable to flush stdout");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let output = buffer
            .strip_suffix("\r\n")
            .or(buffer.strip_suffix("\n"))
            .unwrap_or(&buffer);
        println!("Was this your command: \"{}\"?", output);
    }
}
