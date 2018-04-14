#!/bin/sh

function _installed() {
    VERSION=$($@ --version 2>/dev/null)
    [ -z $? ] && echo $VERSION || echo "none"
}

function _latest() {
    VERSION=$(cargo search -q $@ | grep $@ | cut -f2 -d"\"")
    echo $VERSION
}

echo -n "Fetching latest available 'cargo-make' version... "
INSTALLED=$(_installed cargo-make)
LATEST=$(_latest cargo-make)
echo "${LATEST} (installed: ${INSTALLED})"

if [ $INSTALLED == $LATEST ]; then
  echo "Using latest `cargo-make` (${INSTALLED})"
else
  echo "Installing latest `cargo-make` (${INSTALLED})"
  URL="https://bintray.com/sagiegurari/cargo-make/download_file?file_path=cargo-make_v${LATEST}"
  curl -SsL $URL > $HOME/.cargo/bin
fi
