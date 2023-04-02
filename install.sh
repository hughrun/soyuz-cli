#!/bin/sh

# This script installs soyuz-cli

# Download latest binary
wget -q --show-progress https://github.com/hughrun/soyuz-cli/releases/latest/download/MacOS
# mv to /usr/local/bin and rename
mv MacOS /usr/local/bin/soyuz
# make it executable
chmod +x /usr/local/bin/soyuz

cat 1>&2 << 'EOM'

🚀  soyuz-cli is now installed!

  🔧 Get set up with 'soyuz settings'
  ℹ️  For help try 'soyuz help'

EOM