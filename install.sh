#!/bin/sh

# This script installs soyuz-cli

# Get the latest version number
url=$(curl $1 -s -L -I -o /dev/null -w '%{url_effective}' https://github.com/hughrun/soyuz-cli/releases/latest)
version=${url##*/}  # retain the part after the last slash
# Download latest binary and save in /usr/local/bin
curl -L https://github.com/hughrun/soyuz-cli/releases/download/$version/MacOS > /usr/local/bin/soyuz
# make it executable
chmod +x /usr/local/bin/soyuz

cat 1>&2 << 'EOM'

đ  soyuz-cli is now installed!

  đ§ Get set up with 'soyuz settings'
  âšī¸  For help try 'soyuz help'

EOM