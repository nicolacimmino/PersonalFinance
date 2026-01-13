import express from 'express';
import { apiKeyAuth } from '../../../middleware/apiKeyAuth';
import * as controller from './category.controller.v2';

const router = express.Router();

// GET /api/v2/categories
router.get('/categories', apiKeyAuth, controller.getCategories);

export default router;
