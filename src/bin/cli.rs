use base64::{classic::Standard, decode, encode};
use std::env::args;

fn main() {
    let mut args = args().skip(1);
    let cmd = args.next().expect("Neither `encode` nor `decode` found");
    let text = args.next().expect("No text");

    match cmd.as_str() {
        "encode" => println!("{}", encode(text.as_bytes(), &Standard)),
        "decode" => {
            let origin = decode(&text, &Standard).unwrap();
            let origin = String::from_utf8(origin).unwrap();
            println!("{origin}");
        }
        _ => eprintln!("Unknown subcommand `{cmd}`"),
    }
}
