use std::env;
use wordie::{io, Editor};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "demo" {
        let mut ed = Editor::from_str("Hello World");
        ed.move_left(); ed.move_left(); // place cursor before 'r'
        ed.insert_str("! ");            // "Hello Wo! rld"
        io::save_to_file(&ed, "out.txt")?;
        println!("Wrote: {}", ed.text());
    } else {
        println!("Run: cargo run -- demo");
    }
    Ok(())
}
