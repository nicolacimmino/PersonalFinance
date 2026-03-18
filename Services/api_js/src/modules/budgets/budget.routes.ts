import express from 'express';
import { apiKeyAuth } from '../../middleware/apiKeyAuth';
import * as controller from './budget.controller';

const router = express.Router();

// GET /api/budgets
router.get('/budgets', apiKeyAuth, controller.getBudgets);

export default router;
