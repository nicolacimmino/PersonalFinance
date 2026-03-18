#!/bin/bash

set -e

if [ "$1" != "dev" ] && [ "$1" != "pro" ]; then
    echo "Usage: ./deploy.sh <dev|pro>"
    exit 1
fi

echo "Building..."
npm run build:prod

echo "Deploying to /k/pfinance/$1/obt..."
mkdir -p /k/pfinance/$1/obt
cp dist_prod/index.js /k/pfinance/$1/obt/obt.js

echo "Done!"
