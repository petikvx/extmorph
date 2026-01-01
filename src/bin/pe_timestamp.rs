use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <PE file path>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    println!("🔧 PE Timestamp Updater");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📂 File: {}", filename);

    extmorph::utils::update_pe_timestamp(filename)?;

    println!("\n✅ Done!");

    Ok(())
}
