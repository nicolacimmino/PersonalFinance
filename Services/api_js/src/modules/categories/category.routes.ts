import express from 'express';
import { apiKeyAuth } from '../../middleware/apiKeyAuth';
import * as controller from './category.controller';

const router = express.Router();

// GET /api/categories
router.get('/categories', apiKeyAuth, controller.getCategories);

export default router;
