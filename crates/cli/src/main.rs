#![forbid(unsafe_code)]

mod specanchor;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("specanchor") => specanchor::run(&args[1..]),
        _ => Ok(format!("vaultcore-cli {}", vaultcore_core::VERSION)),
    };

    match result {
        Ok(message) => println!("{message}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
