#!/usr/bin/env bash
set -e

# Determine platform and architecture
case "$(uname -s)-$(uname -m)" in
  Darwin-arm64)
    FILE="mejiro-cli-aarch64-apple-darwin.tar.gz"
    ;;
  Darwin-x86_64)
    FILE="mejiro-cli-x86_64-apple-darwin.tar.gz"
    ;;
  Linux-x86_64)
    FILE="mejiro-cli-x86_64-unknown-linux-gnu.tar.gz"
    ;;
  *)
    echo "Unsupported OS or architecture: $(uname -s)-$(uname -m)"
    exit 1
    ;;
esac

echo "Downloading: $FILE ..."
curl -L -O "https://github.com/tsugumi-sys/mejiro/releases/latest/download/$FILE"

echo "Extracting..."
tar -xzf "$FILE"

echo "Installing to /usr/local/bin/ ..."
sudo mv mejiro-cli /usr/local/bin/

echo "✅ Installation complete! Run 'mejiro-cli --help' to see usage."
