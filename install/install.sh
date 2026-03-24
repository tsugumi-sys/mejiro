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

INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "Installing to ${INSTALL_DIR}/ ..."
mv mejiro-cli "${INSTALL_DIR}/mejiro-cli"
chmod 755 "${INSTALL_DIR}/mejiro-cli"

echo "Cleaning up..."
rm "$FILE"

echo "✅ Installation complete! Run 'mejiro-cli --help' to see usage."

case ":$PATH:" in
  *":${INSTALL_DIR}:"*)
    ;;
  *)
    echo "Add ${INSTALL_DIR} to your PATH if it is not already available in new shells."
    ;;
esac
