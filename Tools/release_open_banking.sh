rsync "/mnt/c/Users/nicola/git/PersonalFinance/OpenBanking/" "OpenBanking" --exclude "target" --delete -avu
cd OpenBanking
cargo build --release
cd ..
scp -i nicola-pfinance.pem OpenBanking/target/release/open_banking ec2-user@ec2-18-159-149-97.eu-central-1.compute.amazonaws.com:~/pfinance-prod/open-banking/
