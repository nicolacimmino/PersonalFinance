import express from 'express';
import { apiKeyAuth } from '../../../middleware/apiKeyAuth';
import * as controller from './receipt.controller.v2';

const router = express.Router();

// GET /api/v2/receipts
router.get('/receipts', apiKeyAuth, controller.getReceipts);

// GET /api/v2/receipts/:id
router.get('/receipts/:id', apiKeyAuth, controller.getReceipt);

// GET /api/v2/receipts/:id/pdf
router.get('/receipts/:id/pdf', apiKeyAuth, controller.getReceiptPdf);

export default router;
