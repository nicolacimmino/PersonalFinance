import express from 'express';
import { apiKeyAuth } from '../../../middleware/apiKeyAuth';
import * as controller from './transaction.controller.v2';

const router = express.Router();

// GET /api/v2/transactions
router.get('/transactions', apiKeyAuth, controller.getTransactions);

// GET /api/v2/transactions/:id
router.get('/transactions/:id', apiKeyAuth, controller.getTransaction);

// POST /api/v2/transactions
router.post('/transactions', apiKeyAuth, controller.createTransaction);

// PATCH /api/v2/transactions/:id
router.patch('/transactions/:id', apiKeyAuth, controller.patchTransaction);

export default router;
