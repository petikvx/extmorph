#!/bin/bash

# Build script for extmorph

echo "🔨 Building extmorph..."

# Build in release mode
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "📦 Binaries created:"
    echo "  • extmorph:      target/release/extmorph"
    echo "  • pe_timestamp:  target/release/pe_timestamp"
    echo ""
    echo "Run with: ./target/release/extmorph --help"
    echo "          ./target/release/pe_timestamp <file.exe>"
else
    echo "❌ Build failed!"
    exit 1
fi
