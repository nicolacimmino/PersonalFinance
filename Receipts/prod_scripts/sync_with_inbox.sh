
aws s3 sync s3://pfinance-receipts/receipts-documents/ s3-pfinance-receipts/
aws s3 sync s3://pfinance-receipts/receipts-documents-processed/ s3-pfinance-receipts-processed/

# Prevents empty folder breaking the file loop.
shopt -s nullglob dotglob

for file in s3-pfinance-receipts/*
do
  echo "$file"
  ./receipts_import $file
  mv $file s3-pfinance-receipts-processed/
done

aws s3 sync s3-pfinance-receipts/ s3://pfinance-receipts/receipts-documents/ --delete
aws s3 sync s3-pfinance-receipts-processed/ s3://pfinance-receipts/receipts-documents-processed/
