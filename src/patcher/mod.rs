use anyhow::{Context, Result};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

use crate::profiles::RansomProfile;

pub struct Patcher {
    profile: RansomProfile,
}

impl Patcher {
    pub fn new(profile: RansomProfile) -> Self {
        Self { profile }
    }

    /// Read and validate the input file
    pub fn read_and_validate(&self, path: &Path) -> Result<Vec<u8>> {
        let content = fs::read(path).context("Failed to read input file")?;

        // Validate SHA-256 if expected hash is provided
        if !self.profile.expected_sha256.is_empty() {
            let hash = self.calculate_sha256(&content);
            if hash != self.profile.expected_sha256 {
                anyhow::bail!(
                    "SHA-256 mismatch!\nExpected: {}\nGot: {}",
                    self.profile.expected_sha256,
                    hash
                );
            }
            println!("✓ SHA-256 validation passed");
        }

        Ok(content)
    }

    /// Modify the binary at specified offsets
    pub fn morph(&self, mut content: Vec<u8>) -> Result<Vec<u8>> {
        let mut rng = rand::thread_rng();

        for &offset in &self.profile.offsets {
            if offset >= content.len() {
                anyhow::bail!("Offset 0x{:X} is out of bounds (file size: {})", offset, content.len());
            }

            // Generate random alphanumeric character
            let random_char = match rng.gen_range(0..62) {
                n @ 0..=9 => (b'0' + n) as u8,
                n @ 10..=35 => (b'A' + n - 10) as u8,
                n @ 36..=61 => (b'a' + n - 36) as u8,
                _ => unreachable!(),
            };

            content[offset] = random_char;
        }

        println!("✓ Modified {} offsets", self.profile.offsets.len());
        Ok(content)
    }

    /// Calculate SHA-256 hash
    pub fn calculate_sha256(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }

    /// Write output file with SHA-256 as filename
    pub fn write_output(&self, content: &[u8]) -> Result<String> {
        let hash = self.calculate_sha256(content);
        let temp_filename = format!(".tmp_{}.exe", rand::random::<u32>());
        let output_filename = format!("{}.exe", hash);

        // Write to temporary file first
        fs::write(&temp_filename, content)
            .context("Failed to write temporary file")?;

        // Rename to final name (this creates a new file with current timestamp)
        fs::rename(&temp_filename, &output_filename)
            .context("Failed to rename to final output file")?;

        // Additionally set mtime and atime to current time
        let now = filetime::FileTime::now();
        filetime::set_file_times(&output_filename, now, now)
            .context("Failed to set file times")?;

        // Update PE TimeDateStamp in the binary header
        if let Err(e) = crate::utils::update_pe_timestamp(&output_filename) {
            eprintln!("⚠️  Warning: Failed to update PE timestamp: {}", e);
        }

        println!("✓ SHA-256: {}", hash);
        println!("✓ Output file: {}", output_filename);
        Ok(output_filename)
    }
}
