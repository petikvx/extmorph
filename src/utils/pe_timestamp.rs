use anyhow::{Context, Result};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::time::{SystemTime, UNIX_EPOCH};

/// Update the TimeDateStamp field in a PE file header
pub fn update_pe_timestamp(filename: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename)
        .context("Failed to open file for PE timestamp update")?;

    // Read PE header offset from position 0x3C (60 bytes)
    file.seek(SeekFrom::Start(0x3C))
        .context("Failed to seek to PE offset position")?;

    let mut pe_offset_bytes = [0u8; 4];
    file.read_exact(&mut pe_offset_bytes)
        .context("Failed to read PE offset")?;
    let pe_offset = u32::from_le_bytes(pe_offset_bytes);

    // Seek to PE header
    file.seek(SeekFrom::Start(pe_offset as u64))
        .context("Failed to seek to PE header")?;

    // Read PE signature (4 bytes: "PE\0\0")
    let mut signature = [0u8; 4];
    file.read_exact(&mut signature)
        .context("Failed to read PE signature")?;

    // Verify PE signature
    if &signature != b"PE\0\0" {
        anyhow::bail!("Invalid PE signature");
    }

    // Skip Machine (2 bytes) and NumberOfSections (2 bytes)
    file.seek(SeekFrom::Current(4))
        .context("Failed to skip to TimeDateStamp")?;

    // Get current Unix timestamp
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Failed to get current time")?
        .as_secs() as u32;

    // Write new TimeDateStamp (4 bytes)
    file.write_all(&now.to_le_bytes())
        .context("Failed to write new TimeDateStamp")?;

    println!("✓ PE TimeDateStamp updated to: {} ({})", now, format_timestamp(now));

    Ok(())
}

/// Format Unix timestamp to human-readable date
fn format_timestamp(timestamp: u32) -> String {
    use std::time::{Duration, UNIX_EPOCH};
    let dt = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    format!("{:?}", dt)
}
