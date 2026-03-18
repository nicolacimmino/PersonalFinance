import express from 'express';
import { apiKeyAuth } from '../../middleware/apiKeyAuth';
import * as controller from './receipt.controller';

const router = express.Router();

// GET /api/receipts
router.get('/receipts', apiKeyAuth, controller.getReceipts);

// GET /api/receipts/:id
router.get('/receipts/:id', apiKeyAuth, controller.getReceipt);

// GET /api/receipts/:id/pdf
router.get('/receipts/:id/pdf', apiKeyAuth, controller.getReceiptPdf);

export default router;
