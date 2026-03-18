import express from 'express';
import { apiKeyAuth } from '../../middleware/apiKeyAuth';
import * as controller from './report.controller';

const router = express.Router();

// GET /api/reports/by_category
router.get('/reports/by_category', apiKeyAuth, controller.getReportByCategory);

// GET /api/reports/indicators
router.get('/reports/indicators', apiKeyAuth, controller.getIndicators);

export default router;
