#!/bin/bash

set -e

if [ "$1" != "dev" ] && [ "$1" != "pro" ]; then
    echo "Usage: ./deploy.sh <dev|pro>"
    exit 1
fi

echo "Building..."
if [ "$1" == "dev" ]; then
    npm run build-devk8
else
    npm run build
fi

echo "Deploying to /k/pfinance/$1/web..."
mkdir -p /k/pfinance/$1/web
cp -r dist/* /k/pfinance/$1/web/

echo "Done!"
