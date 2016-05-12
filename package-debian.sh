#!/bin/bash
# Package debian assets


echo Packaging debian assets
mkdir -v deploy
tar -C build -czf deploy/debian.tar.gz debian

