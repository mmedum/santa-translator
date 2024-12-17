use clap::Parser;
use santa_translator::process_input;

/// Simple program for talking in nisse language
#[derive(Parser)]
struct Cli {
    /// Input to work on
    #[arg(short, long)]
    input: String,

    /// Whether is should decrypt the input
    #[arg(short, long, action)]
    decryption: bool,
}

/// Santa Translator for communicating in encrypted nisse language
fn main() {
    let args = Cli::parse();
    let input = &args.input;
    let decryption = &args.decryption;

    let output = process_input(input, *decryption);
    println!("Nisse output: {}", output);
}
