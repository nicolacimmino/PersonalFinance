import express from 'express';
import { apiKeyAuth } from '../../middleware/apiKeyAuth';
import * as controller from './alert.controller';

const router = express.Router();

// GET /api/alerts
router.get('/alerts', apiKeyAuth, controller.getAlerts);

export default router;
