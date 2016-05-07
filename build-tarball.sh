#!/bin/bash
# Builds tarball for Open Build Service
# OBS build VMs have **no internet** gasp omg

if [ "$1" == "" ]; then
  echo "Please provide a version like 0.1.0"
  exit 1
fi

echo Cleaning up after you....
rm -rvf *.tar.gz wsta-*
rm -rf ~/.cargo
cargo clean

echo Fetching offline assets
cargo fetch

echo Creating tarball
mkdir wsta-$1
cp -rv * wsta-$1
cp -r ~/.cargo wsta-$1
tar -czf $1.tar.gz wsta-$1

rm -rf wsta-$1

echo Done!
