# Assuming the web is served as a static site from an S3 bucket.
# Create a .envrc with an export WEB_S3=s3://your_bucket_name
source ./.envrc

npm run build

aws s3 sync dist/ $WEB_S3 --acl public-read --delete

