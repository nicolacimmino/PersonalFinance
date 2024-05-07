#!/bin/bash

#
# Create a .envrc file in the same folder of this script with the following (uncommented!)
#
# export PFINANCE_PROD_HOST=[your username@host]
# export PFINANCE_PROD_PEMFILE=[your PEM file]
# export PFINANCE_PROD_OB_PATH=[your destination path on the prod machine for OpenBanking]
# export PFINANCE_PROD_SP_PATH=[your destination path on the prod machine for Spendee]

source .envrc

rsync "/mnt/c/Users/nicola/git/PersonalFinance/Spendee/" "Spendee" --exclude "target" --delete -avu
cd Spendee
cargo build --release
cd ..
scp -i $PFINANCE_PROD_PEMFILE Spendee/target/release/spendee $PFINANCE_PROD_HOST:$PFINANCE_PROD_SP_PATH
