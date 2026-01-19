use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty() {
        eprintln!("Usage: claux <prompt>");
        std::process::exit(1);
    }
    
    let prompt = args.join(" ");
    
    let status = Command::new("claude")
        .arg("--dangerously-skip-permissions")
        .arg("-p")
        .arg(&prompt)
        .status()
        .expect("Failed to execute claude command");
    
    std::process::exit(status.code().unwrap_or(1));
}
