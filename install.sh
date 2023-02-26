#!/bin/sh

# This script installs soyuz-cli
# Download latest binary and save in /usr/local/bin
curl -L https://github.com/hughrun/soyuz-cli/releases/download/latest/MacOS > /usr/local/bin/soyuz
# make it executable
chmod +x /usr/local/bin/yawp_test

cat 1>&2 << 'EOM'

🚀  soyuz-cli is now installed!

  🔧 Get set up with 'soyuz settings'
  ℹ️  For help try 'soyuz help'

EOM