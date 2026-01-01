# extmorph

🔧 **Ransomware Extension Morphing Tool** for security research and malware analysis

## 📋 Table of Contents

- [Description](#description)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [List Profiles](#list-profiles)
  - [Show Profile Details](#show-profile-details)
  - [Morph a Binary](#morph-a-binary)
- [Profiles](#profiles)
- [How It Works](#how-it-works)
- [Output Files](#output-files)
- [Examples](#examples)
- [Requirements](#requirements)
- [Troubleshooting](#troubleshooting)
- [Disclaimer](#disclaimer)

## Description

`extmorph` is a specialized tool designed for malware researchers and security analysts to create morphed variants of ransomware binaries. It modifies specific byte offsets in the executable to change the encrypted file extension while preserving the malware's core functionality. This creates polymorphic variants with different SHA-256 hashes, useful for:

- **Antivirus Testing**: Evaluate AV detection capabilities against variants
- **Malware Analysis**: Study behavioral changes based on extension modifications
- **Security Research**: Generate samples for honeypot and sandbox testing
- **Training**: Educational purposes in controlled environments

## Features

- 🎯 **5 Pre-configured Profiles**: Abyss, Babuk, BlackHunt2, Chaos v5.2, Trigona
- 🔐 **SHA-256 Validation**: Ensures you're modifying the correct original sample
- 🎲 **Random Mutations**: Generates unique alphanumeric characters at specified offsets
- 📦 **UPX Compression**: Automatic compression for supported profiles (optional for others)
- ⏱️ **Timestamp Morphing**: Updates both PE header and filesystem timestamps to current time
- 🔄 **PE Header Modification**: Automatically updates TimeDateStamp in PE executable header
- 🏷️ **SHA-256 Filenames**: Output files automatically named with their hash
- ⚡ **Fast & Memory Safe**: Written in Rust for performance and reliability
- 📊 **Detailed Output**: Clear progress indicators and file information
- 🛡️ **Safe Operations**: Validates inputs before making modifications
- 🧰 **Standalone PE Timestamp Tool**: Included utility to update any PE file timestamp

## Installation

### From Source

```bash
# Clone or navigate to the project
cd /path/to/extmorph

# Build in release mode (optimized)
cargo build --release

# The binary will be at target/release/extmorph
# Additional tool: target/release/pe_timestamp
```

### Quick Build Script

```bash
./build.sh
```

### System-wide Installation (Optional)

```bash
# Copy binaries to local bin
sudo cp target/release/extmorph /usr/local/bin/
sudo cp target/release/pe_timestamp /usr/local/bin/

# Now you can run from anywhere
extmorph --help
pe_timestamp <file.exe>
```

## 📂 Setup Malware Directory

For easier usage, create a `malware/` directory and place your samples there:

```bash
# Create directory (already exists)
mkdir -p malware

# Copy your malware samples with proper naming
cp /path/to/abyss-sample.exe malware/abyss-orig.exe
cp /path/to/babuk-sample.exe malware/babuk-orig.exe
# ... etc
```

**Supported naming patterns:**
- `<profile>.exe` → `abyss.exe`, `babuk.exe`
- `<profile>-orig.exe` → `abyss-orig.exe`, `babuk-orig.exe`
- `<profile>_orig.exe` → `abyss_orig.exe`, `babuk_orig.exe`
- `<profile>.bin` → `abyss.bin`, `babuk.bin`

See [malware/README.md](malware/README.md) for detailed instructions and expected SHA-256 hashes.

## Tools

This project includes two binaries:

### 1. extmorph (Main Tool)

The primary ransomware morphing tool with profile-based modifications.

### 2. pe_timestamp (Standalone Utility)

A standalone tool to update the PE TimeDateStamp header of any Windows executable:

```bash
# Update PE timestamp to current time
./target/release/pe_timestamp sample.exe
```

**Output:**
```
🔧 PE Timestamp Updater
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📂 File: sample.exe
✓ PE TimeDateStamp updated to: 1735584000
✅ Done!
```

This tool is automatically called by extmorph but can also be used independently.

## Usage

### List Profiles

Display all available ransomware profiles:

```bash
extmorph list
```

**Output:**
```
📋 Available profiles:

  • abyss - 5 offsets [UPX]
  • babuk - 5 offsets
  • blackhunt2 - 19 offsets
  • chaos52 - 29 offsets
  • trigona - 6 offsets [UPX]
```

### Show Profile Details

Get detailed information about a specific profile:

```bash
extmorph info <profile_name>
```

**Example:**
```bash
extmorph info abyss
```

**Output:**
```
📋 Profile: abyss
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SHA-256: 9243bdcbe30fbd430a841a623e9e1bcc894e4fdc136d46e702a94dad4b10dfdc
Offsets: 5 positions
UPX Compression: Yes

Offsets:
  0x978A, 0x978C, 0x978E, 0x9790, 0x9792
```

### Morph a Binary

Create a morphed variant of a ransomware binary:

```bash
extmorph morph --profile <profile> [--input <file>]
```

#### Method 1: Automatic Discovery (Recommended)

Place your malware samples in the `malware/` directory with naming pattern:
- `<profile>.exe` or `<profile>-orig.exe`

Then simply run:

```bash
# Automatically finds malware/abyss.exe or malware/abyss-orig.exe
extmorph morph --profile abyss

# Short version
extmorph morph -p babuk

# With compression
extmorph morph -p trigona --compress
```

#### Method 2: Manual Path

Specify a custom file path:

```bash
extmorph morph --profile abyss --input /path/to/abyss-orig.exe

# Using short flags
extmorph morph -p trigona -i /custom/path/sample.exe
```

**Output Example:**
```
🔧 extmorph v0.1.0 - Extension Morphing Tool
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Profile: abyss
🔍 Searching for malware file in malware/ directory...
📂 Input: malware/abyss-orig.exe
✓ SHA-256 validation passed
✓ Modified 5 offsets
✓ PE TimeDateStamp updated to: 1735584651 (SystemTime { tv_sec: 1735584651, tv_nsec: 0 })
✓ SHA-256: ff9023694c52a2c84cee53494877e4ef92c0d142618db0fe85cd842483ca71c6
✓ Output file: ff9023694c52a2c84cee53494877e4ef92c0d142618db0fe85cd842483ca71c6.exe
⏳ Compressing with UPX -9...
✓ PE TimeDateStamp updated to: 1735584652 (SystemTime { tv_sec: 1735584652, tv_nsec: 0 })
✓ Compressed: a8d9f3b2e1c7d6f5e4b3c2d1f0e9d8c7b6a5f4e3d2c1b0a9f8e7d6c5b4a3f2e1.exe

✅ Done! Compressed file: a8d9f3b2e1c7d6f5e4b3c2d1f0e9d8c7b6a5f4e3d2c1b0a9f8e7d6c5b4a3f2e1.exe
```

## Profiles

| Profile      | Ransomware   | Offsets | UPX | SHA-256 Check | Description                          |
|--------------|--------------|---------|-----|---------------|--------------------------------------|
| **abyss**    | Abyss        | 5       | ✓   | ✓             | Modifies extension string offsets    |
| **babuk**    | Babuk        | 5       | ✗   | ✓             | Targets encryption extension bytes   |
| **blackhunt2** | BlackHunt2 | 19      | ✗   | ✓             | Multiple extension modification zones |
| **chaos52**  | Chaos v5.2   | 29      | ✗   | ✗             | No SHA validation, extensive offsets |
| **trigona**  | Trigona      | 6       | ✓   | ✓             | Small footprint with compression     |

### Profile-Specific SHA-256 Hashes

Each profile expects a specific original file:

- **abyss**: `9243bdcbe30fbd430a841a623e9e1bcc894e4fdc136d46e702a94dad4b10dfdc`
- **babuk**: `9479a5dc61284ccc3f063ebb38da9f63400d8b25d8bca8d04b1832f02fac24de`
- **blackhunt2**: `74df3452a6b9dcdba658af7a9cf5afb09cce51534f9bc63079827bf73075243b`
- **chaos52**: *(no validation)*
- **trigona**: `85f4088286ac1eedc94ad9dc6465e9e4b89d1cde3012f9949450fcc9f2b60431`

## How It Works

1. **Input Validation**: Checks if the file exists and verifies its SHA-256 hash (if required by profile)
2. **Offset Modification**: Replaces bytes at specified offsets with random alphanumeric characters (a-z, A-Z, 0-9)
3. **File Creation**: Creates temporary file and renames it to ensure fresh filesystem timestamps
4. **PE Timestamp Update**: Modifies the TimeDateStamp field in the PE header to current Unix timestamp
5. **Filesystem Timestamps**: Sets both access time (atime) and modification time (mtime) to current time
6. **Hash Calculation**: Computes the SHA-256 hash of the modified binary
7. **File Naming**: Renames the output file using its SHA-256 hash as the filename
8. **Compression** (optional): Compresses with UPX using maximum compression (-9), then repeats steps 4-7

### What Gets Modified?

**Binary Modifications:**
- **Extension Strings**: The encrypted file extension string embedded in the ransomware binary
  - Example: `.abyss` → `.AbY5s` or `.a8Yss` (random alphanumeric)
  
**Metadata Modifications:**
- **PE TimeDateStamp**: The compilation timestamp in the PE header (offset varies by binary)
- **File System Times**: Access time and modification time on disk

### Timestamp Details

The tool modifies three types of timestamps:

1. **PE Header TimeDateStamp** (inside binary):
   - Located at offset `PE_offset + 8` in the COFF header
   - Set to current Unix timestamp (seconds since 1970-01-01)
   - This is what tools like `exiftool` or `PEview` show as "Time Date Stamp"

2. **File Modification Time** (filesystem metadata):
   - Standard mtime updated to current time
   
3. **File Access Time** (filesystem metadata):
   - Standard atime updated to current time

This comprehensive timestamp modification ensures that forensic analysis shows the file as recently created rather than revealing the original compilation date.

## Output Files

Generated files are named using their SHA-256 hash:

```
<sha256_hash>.exe              # Modified binary (morphed)
<sha256_hash>.exe              # Compressed binary (if applicable)
```

### File Properties

**✅ Updated Automatically:**
- **SHA-256 Filename**: Output file is named with its SHA-256 hash
- **PE TimeDateStamp**: Binary header timestamp set to current time
- **File System Timestamps**: Access and modification times set to current time

This ensures that:
- Each morphed variant has a unique filename based on its hash
- The PE header timestamp reflects when the file was created
- File system metadata shows current timestamps instead of original file's timestamps

**Example:**
```
ff9023694c52a2c84cee53494877e4ef92c0d142618db0fe85cd842483ca71c6.exe
```

## Examples

### Example 1: Basic Morphing (Automatic Discovery)

```bash
# Automatically finds malware/babuk.exe or malware/babuk-orig.exe
./target/release/extmorph morph -p babuk
```

### Example 2: Basic Morphing (Manual Path)

```bash
# Specify custom file path
./target/release/extmorph morph -p babuk -i /custom/path/babuk-orig.exe
```

### Example 3: Batch Processing Multiple Samples

```bash
#!/bin/bash
PROFILES=("abyss" "babuk" "blackhunt2" "chaos52" "trigona")

# Method 1: Using automatic discovery (files in malware/ directory)
for profile in "${PROFILES[@]}"; do
    echo "Processing $profile..."
    ./target/release/extmorph morph -p "$profile"
done

# Method 2: Using custom paths
for profile in "${PROFILES[@]}"; do
    echo "Processing $profile..."
    ./target/release/extmorph morph -p "$profile" -i "/path/to/${profile}-orig.exe"
done
```

### Example 4: Generate Multiple Variants

```bash
# Generate 10 different variants of the same malware (using automatic discovery)
for i in {1..10}; do
    ./target/release/extmorph morph -p abyss
done
```

### Example 5: Force Compression on Non-UPX Profile

```bash
# Babuk doesn't compress by default, but force it (automatic discovery)
./target/release/extmorph morph -p babuk --compress

# Or with manual path
./target/release/extmorph morph -p babuk -i babuk-orig.exe --compress
```

### Example 6: Update PE Timestamp Only

```bash
# Use standalone tool to only update PE timestamp without morphing
./target/release/pe_timestamp sample.exe

# Check the updated timestamp
exiftool sample.exe | grep "Time Date Stamp"
```

### Example 7: Verify Timestamp Changes

```bash
# Before morphing
stat original.exe
exiftool original.exe | grep "Time Date Stamp"

# Morph the binary (using automatic discovery if file is in malware/)
./target/release/extmorph morph -p abyss

# After morphing - check new file
OUTPUT=$(ls -t *.exe | head -1)
stat "$OUTPUT"
exiftool "$OUTPUT" | grep "Time Date Stamp"
```

## Requirements

### Mandatory
- **Rust 1.70+**: For building from source
- **Original malware samples**: With correct SHA-256 hashes (except chaos52)

### Optional
- **UPX**: Required for compression features
  ```bash
  # Install UPX on Linux
  sudo apt install upx-ucl
  
  # Or download from https://upx.github.io/
  ```

- **exiftool**: For verifying PE timestamp changes (optional)
  ```bash
  # Install exiftool
  sudo apt install libimage-exiftool-perl
  
  # Check PE timestamp
  exiftool file.exe | grep "Time Date Stamp"
  ```

## Troubleshooting

### Error: SHA-256 mismatch

```
Error: SHA-256 mismatch!
Expected: 9243bdcbe30fbd430a841a623e9e1bcc894e4fdc136d46e702a94dad4b10dfdc
Got: 1234567890abcdef...
```

**Solution**: Ensure you're using the correct original file for the profile. Verify with:
```bash
sha256sum your-file.exe
```

### Error: Profile not found

```
Error: Profile 'abysss' not found
```

**Solution**: Check available profiles with:
```bash
extmorph list
```

### Error: Malware file not found

```
❌ No file found for profile 'abyss'

📂 Available files in malware/ directory:
  • babuk-orig.exe
  • trigona.exe

Please add malware file to malware/ directory or use -i to specify custom path
```

**Solution**: Add the required file to `malware/` directory with proper naming:
```bash
# Option 1: Add file to malware/ directory
cp /path/to/your-sample.exe malware/abyss-orig.exe

# Option 2: Use manual path
extmorph morph -p abyss -i /path/to/your-sample.exe
```

### Warning: UPX not found

```
⚠️  Warning: UPX not found, skipping compression
```

**Solution**: Install UPX or disable compression:
```bash
sudo apt install upx-ucl
```

### Error: Offset out of bounds

```
Error: Offset 0x978A is out of bounds (file size: 1024)
```

**Solution**: The file is too small or corrupted. Verify file integrity.

### Verifying Timestamp Changes

To verify that PE timestamps were updated:

```bash
# Method 1: Using exiftool
exiftool output.exe | grep "Time Date Stamp"

# Method 2: Using readpe (from pev package)
readpe output.exe | grep "Date"

# Method 3: Check filesystem timestamps
stat output.exe
ls -l output.exe
```

Expected output should show current date/time, not the original compilation date.

### PE Timestamp Tool Shows Error

```
Error: Invalid PE signature
```

**Solution**: Ensure the file is a valid PE executable (Windows .exe or .dll). The tool only works with PE format files.

## Development

### Project Structure

```
extmorph/
├── src/
│   ├── main.rs           # CLI interface
│   ├── lib.rs            # Library exports
│   ├── bin/
│   │   └── pe_timestamp.rs  # Standalone PE timestamp tool
│   ├── patcher/          # Binary modification logic
│   │   └── mod.rs
│   ├── profiles/         # Ransomware profiles
│   │   ├── mod.rs
│   │   ├── abyss.rs
│   │   ├── babuk.rs
│   │   ├── blackhunt2.rs
│   │   ├── chaos52.rs
│   │   └── trigona.rs
│   └── utils/            # Utilities
│       ├── mod.rs
│       └── pe_timestamp.rs  # PE timestamp modification
├── Cargo.toml            # Dependencies
├── build.sh              # Build script
└── README.md
```

### Adding a New Profile

1. Create a new file in `src/profiles/<name>.rs`:
```rust
use super::RansomProfile;

pub fn profile() -> RansomProfile {
    RansomProfile::new(
        "myransomware",
        "sha256hash...",
        vec![0x1000, 0x1002, 0x1004],
        false, // UPX compression
    )
}
```

2. Add to `src/profiles/mod.rs`:
```rust
pub mod myransomware;

// In get_profile():
"myransomware" => Some(myransomware::profile()),

// In list_profiles():
"myransomware".to_string(),
```

## ⚠️ Disclaimer

**FOR SECURITY RESEARCH AND EDUCATIONAL PURPOSES ONLY**

This tool is intended exclusively for:
- Authorized malware analysis in controlled environments
- Security research and antivirus testing
- Educational demonstrations in cybersecurity courses

**DO NOT:**
- Use on systems you don't own or have explicit permission to test
- Distribute morphed samples without proper handling procedures
- Execute generated binaries outside of isolated sandboxes
- Use for malicious purposes

The authors and contributors are not responsible for misuse of this tool. Always follow responsible disclosure practices and comply with local laws and regulations.

## License

This project is provided for security research purposes only. Use responsibly.

## Author

**petikvx** - Malware Researcher & Security Analyst

## Acknowledgments

- Original Python scripts converted to Rust for this project
- Ransomware samples used for research purposes only
- UPX compression tool integration
- PE timestamp modification inspired by C implementation

## Technical Details

### Timestamp Morphing Implementation

The tool implements multi-layer timestamp modification:

**1. PE Header TimeDateStamp (Binary Level)**
- Reads PE offset from position 0x3C
- Navigates to PE header signature ("PE\0\0")
- Updates TimeDateStamp at offset PE+8 with current Unix timestamp
- This timestamp is visible in PE analysis tools

**2. Filesystem Timestamps (OS Level)**
- Creates temporary file first, then renames to final name
- Sets both atime (access time) and mtime (modification time)
- Uses `filetime` crate for cross-platform compatibility

**3. File Naming (Content-Based)**
- SHA-256 hash computed after all modifications
- Filename reflects exact binary content
- Each variant automatically has unique name

This multi-layer approach ensures maximum obfuscation of the original binary's timeline.

### PE Format Structure

```
DOS Header (0x00)
  └─ e_lfanew (0x3C) → PE Header offset

PE Header (variable offset)
  ├─ Signature (4 bytes): "PE\0\0"
  ├─ Machine (2 bytes)
  ├─ NumberOfSections (2 bytes)
  └─ TimeDateStamp (4 bytes) ← Modified by this tool
```

---

**Version**: 0.1.0  
**Last Updated**: December 2025
