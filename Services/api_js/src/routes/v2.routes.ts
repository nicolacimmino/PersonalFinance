import express from 'express';

// Import v2 routes
import categoryRoutesV2 from '../modules/categories/v2/category.routes.v2';
import accountRoutesV2 from '../modules/accounts/v2/account.routes.v2';
import transactionRoutesV2 from '../modules/transactions/v2/transaction.routes.v2';
import reportRoutesV2 from '../modules/reports/v2/report.routes.v2';
import budgetRoutesV2 from '../modules/budgets/v2/budget.routes.v2';
import alertRoutesV2 from '../modules/alerts/v2/alert.routes.v2';
import receiptRoutesV2 from '../modules/receipts/v2/receipt.routes.v2';

const router = express.Router();

// Mount all v2 routes
router.use(categoryRoutesV2);
router.use(accountRoutesV2);
router.use(transactionRoutesV2);
router.use(reportRoutesV2);
router.use(budgetRoutesV2);
router.use(alertRoutesV2);
router.use(receiptRoutesV2);

export default router;
