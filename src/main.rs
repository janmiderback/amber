mod repl;

fn main() {
    repl::run().expect("REPL loop failed");
}
