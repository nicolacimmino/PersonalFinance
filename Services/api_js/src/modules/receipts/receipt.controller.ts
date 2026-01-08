import { Request, Response } from 'express';
import * as service from './receipt.service';
import { ReceiptDto } from './receipt.dto';
import { ApplicationReceipt } from './receipt.model';
import { getReceiptPdf as fetchReceiptPdfFromS3 } from '../../services/awsS3Client';

/**
 * Transform database model to DTO
 */
function toReceiptDto(receipt: ApplicationReceipt): ReceiptDto {
  return {
    id: receipt.id,
    date: receipt.date.toISOString(),
    amount_cents: receipt.amount_cents,
    currency: receipt.currency,
    ext_id: receipt.ext_id,
    merchant_name: receipt.merchant_name,
    merchant_address: receipt.merchant_address,
    scan_file_name: receipt.scan_file_name,
    transaction_id: receipt.transaction_id,
    transaction_category: receipt.transaction_category,
    transaction_amount_cents: receipt.transaction_amount_cents,
    transaction_currency: receipt.transaction_currency,
    account_code: receipt.account_code,
    account_description: receipt.account_description,
  };
}

/**
 * GET /api/receipts
 * Get all receipts
 */
export async function getReceipts(
  _req: Request,
  res: Response
): Promise<void> {
  const receipts = await service.getReceipts();
  const dtos = receipts.map(toReceiptDto);
  res.json(dtos);
}

/**
 * GET /api/receipts/:id
 * Get a single receipt by ID
 */
export async function getReceipt(req: Request, res: Response): Promise<void> {
  const id = parseInt(req.params.id, 10);

  if (isNaN(id)) {
    res.status(400).json({ error: 'Invalid receipt ID' });
    return;
  }

  const receipt = await service.getReceipt(id);
  const dto = toReceiptDto(receipt);
  res.json(dto);
}

/**
 * GET /api/receipts/:id/pdf
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
