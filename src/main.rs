use std::io::Write;

fn main() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        println!("{}", buffer);
    }
}
