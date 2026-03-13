#!/usr/bin/bash

TARGET_DIR=target/debian
VERSION=$(echo $1 | sed -E 's/^v([^-]+).*$/\1/g')

# Setup files
rm -rf $TARGET_DIR
mkdir -p $TARGET_DIR/cubic/
cp -r debian/ $TARGET_DIR/cubic/
cp -r src/ $TARGET_DIR/cubic/
cp Cargo.toml $TARGET_DIR/cubic/
cp Cargo.lock $TARGET_DIR/cubic/
cp Makefile $TARGET_DIR/cubic/

# Set variables
sed -i "s/%version%/$VERSION/g" $TARGET_DIR/cubic/debian/changelog
sed -i "s/%date%/$(date)/g" $TARGET_DIR/cubic/debian/changelog

# Build Source Debian Package
(cd $TARGET_DIR/cubic && dpkg-buildpackage -S -us -uc)
