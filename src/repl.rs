use std::io::{self, Write};

pub fn run() -> io::Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        println!("{}", buffer);
    }
}
