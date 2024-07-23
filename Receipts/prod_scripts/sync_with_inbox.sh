
mkdir -p s3-pfinance-receipts/
mkdir -p s3-pfinance-receipts-processed/
mkdir -p s3-pfinance-receipts-failed/

aws s3 sync s3://pfinance-receipts/receipts-documents/ s3-pfinance-receipts/

# Prevents empty folder breaking the file loop.
shopt -s nullglob dotglob

for file in s3-pfinance-receipts/*
do
  echo "$file"
  if ./receipts_import "$file"; then
      mv "$file" s3-pfinance-receipts-processed/
  else
      mv "$file" s3-pfinance-receipts-failed/
  fi
done

aws s3 sync s3-pfinance-receipts/ s3://pfinance-receipts/receipts-documents/ --delete
aws s3 sync s3-pfinance-receipts-processed/ s3://pfinance-receipts/receipts-documents-processed/
aws s3 sync s3-pfinance-receipts-failed/ s3://pfinance-receipts/receipts-documents-failed/


rm -rf s3-pfinance-receipts/
rm -rf s3-pfinance-receipts-processed/
rm -rf s3-pfinance-receipts-failed/
