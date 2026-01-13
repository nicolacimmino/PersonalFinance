import express from 'express';
import { apiKeyAuth } from '../../../middleware/apiKeyAuth';
import * as controller from './account.controller.v2';

const router = express.Router();

// GET /api/v2/accounts
router.get('/accounts', apiKeyAuth, controller.getAccounts);

// GET /api/v2/accounts/:id
router.get('/accounts/:id', apiKeyAuth, controller.getAccount);

export default router;
