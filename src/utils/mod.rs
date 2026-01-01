use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::process::Command;

pub mod pe_timestamp;

pub use pe_timestamp::update_pe_timestamp;

pub fn compress_upx(input: &str) -> Result<String> {
    let temp_compressed = format!(".tmp_{}.exe", rand::random::<u32>());

    println!("⏳ Compressing with UPX -9...");

    let output = Command::new("upx")
        .args(&["-9", "-o", &temp_compressed, input])
        .output()
        .context("Failed to run UPX. Is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("UPX compression failed: {}", stderr);
    }

    // Calculate SHA256 of compressed file
    let compressed_content = fs::read(&temp_compressed)
        .context("Failed to read compressed file")?;
    let mut hasher = Sha256::new();
    hasher.update(&compressed_content);
    let hash = format!("{:x}", hasher.finalize());
    
    // Rename with SHA256
    let final_compressed = format!("{}.exe", hash);
    fs::rename(&temp_compressed, &final_compressed)
        .context("Failed to rename compressed file")?;

    // Set both access and modification time to current time
    let now = filetime::FileTime::now();
    filetime::set_file_times(&final_compressed, now, now)
        .context("Failed to set file times")?;

    // Update PE TimeDateStamp in the binary header
    if let Err(e) = update_pe_timestamp(&final_compressed) {
        eprintln!("⚠️  Warning: Failed to update PE timestamp: {}", e);
    }

    println!("✓ Compressed: {}", final_compressed);
    Ok(final_compressed)
}

pub fn verify_upx_installed() -> bool {
    Command::new("upx")
        .arg("--version")
        .output()
        .is_ok()
}
