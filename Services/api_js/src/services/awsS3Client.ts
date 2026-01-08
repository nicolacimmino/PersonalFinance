import { S3Client, GetObjectCommand } from '@aws-sdk/client-s3';
import { logger } from '../config/logger';

// Initialize S3 client
const s3Client = new S3Client({
  region: process.env.AWS_REGION || 'eu-central-1',
  credentials: {
    accessKeyId: process.env.AWS_ACCESS_KEY_ID || '',
    secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY || '',
  },
});

const S3_BUCKET = process.env.AWS_S3_BUCKET || 'pfinance-receipts';
const RECEIPTS_PREFIX = 'receipts-documents-processed/';

/**
 * Retrieve a receipt PDF file from S3
 * @param scanFileName The filename of the receipt in S3
 * @returns Buffer containing the PDF file data
 */
export async function getReceiptPdf(scanFileName: string): Promise<Buffer> {
  const key = `${RECEIPTS_PREFIX}${scanFileName}`;

  try {
    logger.debug('Fetching receipt from S3', {
      bucket: S3_BUCKET,
      key,
    });

    const command = new GetObjectCommand({
      Bucket: S3_BUCKET,
      Key: key,
    });

    const response = await s3Client.send(command);

    // Convert stream to buffer
    if (!response.Body) {
      throw new Error('No body in S3 response');
    }

    const chunks: Uint8Array[] = [];
    const stream = response.Body as any;

    for await (const chunk of stream) {
      chunks.push(chunk);
    }

    const buffer = Buffer.concat(chunks);

    logger.info('Successfully retrieved receipt from S3', {
      bucket: S3_BUCKET,
      key,
      size: buffer.length,
    });

    return buffer;
  } catch (error) {
    logger.error('Error retrieving receipt from S3', {
      bucket: S3_BUCKET,
      key,
      error,
    });
    throw new Error(`Failed to retrieve receipt: ${scanFileName}`);
  }
}

export default {
  getReceiptPdf,
};
