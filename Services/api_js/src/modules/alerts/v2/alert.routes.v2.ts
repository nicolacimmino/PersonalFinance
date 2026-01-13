import express from 'express';
import { apiKeyAuth } from '../../../middleware/apiKeyAuth';
import * as controller from './alert.controller.v2';

const router = express.Router();

// GET /api/v2/alerts
router.get('/alerts', apiKeyAuth, controller.getAlerts);

export default router;
