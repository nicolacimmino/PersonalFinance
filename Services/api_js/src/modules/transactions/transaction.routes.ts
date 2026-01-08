import express from 'express';
import { apiKeyAuth } from '../../middleware/apiKeyAuth';
import * as controller from './transaction.controller';

const router = express.Router();

// GET /api/transactions
router.get('/transactions', apiKeyAuth, controller.getTransactions);

// GET /api/transactions/:id
router.get('/transactions/:id', apiKeyAuth, controller.getTransaction);

// POST /api/transactions
router.post('/transactions', apiKeyAuth, controller.createTransaction);

// PATCH /api/transactions/:id
router.patch('/transactions/:id', apiKeyAuth, controller.patchTransaction);

export default router;
