#!/bin/bash
# Builds tarball and assets for Open Build Service
# OBS build VMs have **no internet** gasp omg

check_version_numbers() {

  files=(Cargo.toml build/wsta.spec HomebrewFormula/wsta.rb build/wsta.dsc build/debian/changelog wsta.1)

  for file in "${files[@]}"; do
    echo "$file" "$1"
    grep -q "$1" "$file"
    status=$?
    if [ $status -ne 0 ]; then
      echo ----------------------------------------------
      echo remember to set version "$1" in "$file"
      echo ----------------------------------------------
    fi
  done
}

if [ "$1" == "" ]; then
  echo "Please provide a version like 0.1.0"
  exit 1
fi

check_version_numbers "$1"

echo Cleaning up after you....
rm -rfv deploy
rm -rf ~/.cargo/registry
cargo clean

./package-debian.sh

echo Fetching offline assets
cargo fetch

echo Creating tarball
mkdir -p "deploy/wsta-$1"
cp -rv {Cargo.*,src,wsta.1,README.md,Makefile,LICENCE} "deploy/wsta-$1"
mkdir -v  "deploy/wsta-$1/.cargo"
cp -r ~/.cargo/registry "deploy/wsta-$1/.cargo"
cp -v rust/* "deploy/wsta-$1"
tar -C deploy -czf "deploy/$1.tar.gz" "wsta-$1"
cp "deploy/$1.tar.gz" "deploy/wsta_$1.orig.tar.gz"


rm -rf "deploy/wsta-$1"

echo Done!
