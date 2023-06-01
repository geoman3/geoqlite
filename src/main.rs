use std::io;
use std::io::Write;
use std::process;

fn main() -> io::Result<()> {

    loop {
        print!("db > ");
        io::stdout().flush().expect("Unable to flush stdout");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let command = buffer
            .strip_suffix("\r\n")
            .or(buffer.strip_suffix("\n"))
            .unwrap_or(&buffer);

        if command == ".exit" {
            process::exit(0x00)
        }

        println!("unrecognised command: \"{}\"", command);
    }
}
