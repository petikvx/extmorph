# Quick Start Guide

## 🚀 Setup (First Time)

1. **Build the project:**
   ```bash
   ./build.sh
   ```

2. **Setup malware directory:**
   ```bash
   ./setup_malware.sh
   ```

3. **Add your malware samples to `malware/` directory:**
   ```bash
   cp /path/to/your-sample.exe malware/abyss-orig.exe
   # Or use other naming: abyss.exe, abyss_orig.exe, abyss.bin
   ```

## 📋 Daily Usage

### List available profiles
```bash
./target/release/extmorph list
```

### Morph a binary (automatic discovery)
```bash
./target/release/extmorph morph -p abyss
./target/release/extmorph morph -p babuk --compress
```

### Morph a binary (manual path)
```bash
./target/release/extmorph morph -p abyss -i /custom/path/sample.exe
```

### Show profile details
```bash
./target/release/extmorph info abyss
```

### Update PE timestamp only
```bash
./target/release/pe_timestamp sample.exe
```

## 📂 Directory Structure

```
extmorph/
├── malware/              # Place your malware samples here
│   ├── abyss-orig.exe
│   ├── babuk-orig.exe
│   ├── blackhunt2-orig.exe
│   ├── chaos52-orig.exe
│   └── trigona-orig.exe
├── target/release/
│   ├── extmorph         # Main tool
│   └── pe_timestamp     # PE timestamp utility
├── build.sh             # Build script
└── setup_malware.sh     # Setup helper
```

## 🎯 Common Tasks

### Generate 10 variants of a sample
```bash
for i in {1..10}; do
    ./target/release/extmorph morph -p abyss
done
```

### Process all profiles
```bash
for profile in abyss babuk blackhunt2 chaos52 trigona; do
    ./target/release/extmorph morph -p "$profile"
done
```

### Verify output file timestamp
```bash
OUTPUT=$(ls -t *.exe | head -1)
exiftool "$OUTPUT" | grep "Time Date Stamp"
stat "$OUTPUT"
```

## 📚 More Information

- Full documentation: [README.md](README.md)
- Malware directory setup: [malware/README.md](malware/README.md)
- GitHub Issues: Report bugs or request features

## ⚠️ Security Reminder

- Only run in isolated/sandboxed environment
- Never execute morphed samples on production systems
- Keep malware samples secure and access-controlled
