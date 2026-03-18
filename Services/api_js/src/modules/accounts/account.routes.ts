import express from 'express';
import { apiKeyAuth } from '../../middleware/apiKeyAuth';
import * as controller from './account.controller';

const router = express.Router();

// GET /api/accounts
router.get('/accounts', apiKeyAuth, controller.getAccounts);

// GET /api/accounts/:id
router.get('/accounts/:id', apiKeyAuth, controller.getAccount);

export default router;
