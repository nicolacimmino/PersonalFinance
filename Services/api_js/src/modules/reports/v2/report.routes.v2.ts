import express from 'express';
import { apiKeyAuth } from '../../../middleware/apiKeyAuth';
import * as controller from './report.controller.v2';

const router = express.Router();

// GET /api/v2/reports/by_category
router.get('/reports/by_category', apiKeyAuth, controller.getReportByCategory);

// GET /api/v2/reports/indicators
router.get('/reports/indicators', apiKeyAuth, controller.getIndicators);

export default router;
