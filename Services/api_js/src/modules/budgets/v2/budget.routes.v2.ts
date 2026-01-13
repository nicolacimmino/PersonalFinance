import express from 'express';
import { apiKeyAuth } from '../../../middleware/apiKeyAuth';
import * as controller from './budget.controller.v2';

const router = express.Router();

// GET /api/v2/budgets
router.get('/budgets', apiKeyAuth, controller.getBudgets);

export default router;
