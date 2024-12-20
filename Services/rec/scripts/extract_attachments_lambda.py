import email
import boto3
import json

read_bucket = 'pfinance-receipts'
write_bucket = 'pfinance-receipts'
s3 = boto3.resource('s3')

def lambda_handler(event, context):
    messageID = event['Records'][0]['ses']['mail']['messageId']

    key = f'receipts-inbox/{messageID}'
    read_obj = s3.Object(read_bucket, key)
    body = read_obj.get()['Body'].read().decode('utf8')

    msg = email.message_from_string(body)
    payloads = msg.get_payload()

    for attachment in payloads[1:]:
        name = attachment.get_filename()
        data = attachment.get_payload(decode=True)
        key = f'receipts-documents/{messageID}-{name}'
        s3.Bucket(write_bucket).put_object(Key=key, Body=data)

    read_obj.delete()
    
    return {
        'statusCode': 200,
        'body': json.dumps('Email Processed Successfully')
    }