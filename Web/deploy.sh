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
    npm run build:prod
fi

echo "Deploying to /k/pfinance/$1/web..."
mkdir -p /k/pfinance/$1/web
if [ "$1" == "dev" ]; then
    cp -r dist_devk8/* /k/pfinance/$1/web/
else
    cp -r dist_prod/* /k/pfinance/$1/web/
fi

echo "Done!"
