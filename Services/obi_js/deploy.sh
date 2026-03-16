#!/bin/bash

set -e

if [ "$1" != "dev" ] && [ "$1" != "pro" ]; then
    echo "Usage: ./deploy.sh <dev|pro>"
    exit 1
fi

echo "Building..."
npm run build:prod

echo "Deploying to /k/pfinance/$1/obi..."
mkdir -p /k/pfinance/$1/obi
cp dist_prod/index.js /k/pfinance/$1/obi/obi.js

echo "Done!"
