import { Request, Response } from 'express';
import * as service from '../receipt.service';
import { ReceiptDtoV2 } from './receipt.dto.v2';
import { ApplicationReceipt } from '../receipt.model';
import { getReceiptPdf as fetchReceiptPdfFromS3 } from '../../../services/awsS3Client';

/**
 * Transform database model to V2 DTO (camelCase)
 */
function toReceiptDtoV2(receipt: ApplicationReceipt): ReceiptDtoV2 {
  return {
    id: receipt.id,
    date: receipt.date.toISOString(),
    amountCents: receipt.amount_cents,
    currency: receipt.currency,
    externalId: receipt.ext_id,
    merchantName: receipt.merchant_name,
    merchantAddress: receipt.merchant_address,
    scanFileName: receipt.scan_file_name,
    transactionId: receipt.transaction_id,
    transactionCategory: receipt.transaction_category,
    transactionAmountCents: receipt.transaction_amount_cents,
    transactionCurrency: receipt.transaction_currency,
    accountCode: receipt.account_code,
    accountDescription: receipt.account_description,
  };
}

/**
 * GET /api/v2/receipts
 * Get all receipts
 */
export async function getReceipts(
  _req: Request,
  res: Response
): Promise<void> {
  const receipts = await service.getReceipts();
  const dtos = receipts.map(toReceiptDtoV2);
  res.json(dtos);
}

/**
 * GET /api/v2/receipts/:id
 * Get a single receipt by ID
 */
export async function getReceipt(req: Request, res: Response): Promise<void> {
  const id = parseInt(req.params.id, 10);

  if (isNaN(id)) {
    res.status(400).json({ error: 'Invalid receipt ID' });
    return;
  }

  const receipt = await service.getReceipt(id);
  const dto = toReceiptDtoV2(receipt);
  res.json(dto);
}

/**
 * GET /api/v2/receipts/:id/pdf
 * Download receipt PDF from S3
 */
export async function getReceiptPdf(
  req: Request,
  res: Response
): Promise<void> {
  const id = parseInt(req.params.id, 10);

  if (isNaN(id)) {
    res.status(400).json({ error: 'Invalid receipt ID' });
    return;
  }

  // Get receipt info to retrieve the scan_file_name
  const receipt = await service.getReceipt(id);

  // Fetch PDF from S3
  const pdfBuffer = await fetchReceiptPdfFromS3(receipt.scan_file_name);

  // Set headers and send PDF
  res.setHeader('Content-Type', 'application/pdf');
  res.setHeader(
    'Content-Disposition',
    `attachment; filename="${receipt.scan_file_name}"`
  );
  res.send(pdfBuffer);
}
