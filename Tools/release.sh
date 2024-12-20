
source .envrc

rsync "/mnt/c/Users/nicola/git/PersonalFinance/services/$1/" "$1" --exclude "target" --delete -avu
cd $1
cargo build --release
cd ..
scp -i $PFINANCE_PROD_PEMFILE $1/target/release/$1 $PFINANCE_PROD_HOST:$PFINANCE_PROD_SERVICES_PATH/$1
rm -rf $1