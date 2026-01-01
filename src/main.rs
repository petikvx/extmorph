use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs;

mod patcher;
mod profiles;
mod utils;

use patcher::Patcher;

/// Find malware file in the malware/ directory based on profile name
fn find_malware_file(profile_name: &str) -> Result<PathBuf> {
    let malware_dir = PathBuf::from("malware");
    
    if !malware_dir.exists() {
        anyhow::bail!("Malware directory not found. Please create 'malware/' directory or specify input file with -i");
    }

    // Common filename patterns to try
    let patterns = vec![
        format!("{}.exe", profile_name),
        format!("{}-orig.exe", profile_name),
        format!("{}_orig.exe", profile_name),
        format!("{}.bin", profile_name),
    ];

    // Search for matching files
    for pattern in patterns {
        let path = malware_dir.join(&pattern);
        if path.exists() {
            return Ok(path);
        }
    }

    // If no exact match, list available files
    println!("❌ No file found for profile '{}'", profile_name);
    println!("\n📂 Available files in malware/ directory:");
    
    if let Ok(entries) = fs::read_dir(&malware_dir) {
        let mut found_any = false;
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".exe") || name.ends_with(".bin") {
                    println!("  • {}", name);
                    found_any = true;
                }
            }
        }
        if !found_any {
            println!("  (no .exe or .bin files found)");
        }
    }
    
    anyhow::bail!("Please add malware file to malware/ directory or use -i to specify custom path")
}

#[derive(Parser)]
#[command(name = "extmorph")]
#[command(author = "petikvx")]
#[command(version = "0.1.0")]
#[command(about = "Ransomware extension morphing tool for security research", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Morph a ransomware binary using a specific profile
    Morph {
        /// Profile name (abyss, babuk, blackhunt2, chaos52, trigona)
        #[arg(short, long)]
        profile: String,

        /// Input file path (optional, will search in malware/ directory if not specified)
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Force UPX compression even if profile doesn't require it
        #[arg(short, long)]
        compress: bool,
    },
    /// List all available profiles
    List,
    /// Show profile details
    Info {
        /// Profile name
        profile: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Morph {
            profile: profile_name,
            input,
            compress,
        } => {
            println!("🔧 extmorph v0.1.0 - Extension Morphing Tool");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            // Get profile
            let profile = profiles::get_profile(&profile_name)
                .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found", profile_name))?;

            println!("📋 Profile: {}", profile.name);

            // Determine input file: use provided path or search in malware/ directory
            let input_path = if let Some(path) = input {
                path
            } else {
                println!("🔍 Searching for malware file in malware/ directory...");
                find_malware_file(&profile_name)?
            };

            println!("📂 Input: {}", input_path.display());

            // Check if file exists
            if !input_path.exists() {
                anyhow::bail!("Input file does not exist: {}", input_path.display());
            }

            // Create patcher
            let patcher = Patcher::new(profile.clone());

            // Read and validate
            let content = patcher.read_and_validate(&input_path)?;

            // Morph
            let morphed = patcher.morph(content)?;

            // Write output
            let output_file = patcher.write_output(&morphed)?;

            // Compress if needed
            if profile.compress || compress {
                if !utils::verify_upx_installed() {
                    eprintln!("⚠️  Warning: UPX not found, skipping compression");
                } else {
                    let compressed = utils::compress_upx(&output_file)?;
                    println!("\n✅ Done! Compressed file: {}", compressed);
                    return Ok(());
                }
            }

            println!("\n✅ Done! Output file: {}", output_file);
        }
        Commands::List => {
            println!("📋 Available profiles:\n");
            for profile_name in profiles::list_profiles() {
                let profile = profiles::get_profile(&profile_name).unwrap();
                let compress_flag = if profile.compress { " [UPX]" } else { "" };
                println!(
                    "  • {} - {} offsets{}",
                    profile.name,
                    profile.offsets.len(),
                    compress_flag
                );
            }
        }
        Commands::Info { profile: profile_name } => {
            let profile = profiles::get_profile(&profile_name)
                .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found", profile_name))?;

            println!("📋 Profile: {}", profile.name);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            if !profile.expected_sha256.is_empty() {
                println!("SHA-256: {}", profile.expected_sha256);
            } else {
                println!("SHA-256: (no validation)");
            }
            println!("Offsets: {} positions", profile.offsets.len());
            println!("UPX Compression: {}", if profile.compress { "Yes" } else { "No" });
            println!("\nOffsets:");
            for (i, offset) in profile.offsets.iter().enumerate() {
                if i % 5 == 0 {
                    print!("  ");
                }
                print!("0x{:X}", offset);
                if i < profile.offsets.len() - 1 {
                    print!(", ");
                }
                if (i + 1) % 5 == 0 {
                    println!();
                }
            }
            println!();
        }
    }

    Ok(())
}
