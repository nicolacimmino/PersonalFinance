import express from 'express';

// Import existing v1 routes
import categoryRoutes from '../modules/categories/category.routes';
import accountRoutes from '../modules/accounts/account.routes';
import transactionRoutes from '../modules/transactions/transaction.routes';
import reportRoutes from '../modules/reports/report.routes';
import budgetRoutes from '../modules/budgets/budget.routes';
import alertRoutes from '../modules/alerts/alert.routes';
import receiptRoutes from '../modules/receipts/receipt.routes';

const router = express.Router();

// Mount all v1 routes
router.use(categoryRoutes);
router.use(accountRoutes);
router.use(transactionRoutes);
router.use(reportRoutes);
router.use(budgetRoutes);
router.use(alertRoutes);
router.use(receiptRoutes);

export default router;
