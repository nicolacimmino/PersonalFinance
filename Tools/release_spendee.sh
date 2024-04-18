rsync "/mnt/c/Users/nicola/git/PersonalFinance/Spendee/" "Spendee" --exclude "target" --delete -avu
cd Spendee
cargo build --release
cd ..
scp -i nicola-pfinance.pem Spendee/target/release/spendee ec2-user@ec2-18-159-149-97.eu-central-1.compute.amazonaws.com:~/pfinance-prod/spendee/
