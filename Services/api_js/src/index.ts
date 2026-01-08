import 'express-async-errors';
import express, { Application } from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import { logger } from './config/logger';
import { closePool } from './config/database';
import { errorHandler, notFoundHandler } from './middleware/errorHandler';

// Import route modules
import categoryRoutes from './modules/categories/category.routes';
import accountRoutes from './modules/accounts/account.routes';
import transactionRoutes from './modules/transactions/transaction.routes';
import reportRoutes from './modules/reports/report.routes';
import budgetRoutes from './modules/budgets/budget.routes';
import alertRoutes from './modules/alerts/alert.routes';
import receiptRoutes from './modules/receipts/receipt.routes';

// Load environment variables
dotenv.config();

const app: Application = express();
const PORT = process.env.PORT || 8001;

// Middleware
app.use(cors()); // Enable CORS for all origins (matching Rust API)
app.use(express.json()); // Parse JSON request bodies
app.use(express.urlencoded({ extended: true })); // Parse URL-encoded bodies

// Request logging middleware
app.use((req, _res, next) => {
  logger.info(`${req.method} ${req.path}`, {
    query: req.query,
    ip: req.ip,
  });
  next();
});

// Health check endpoint (no auth required)
app.get('/health', (_req, res) => {
  res.json({ status: 'ok' });
});

// Mount API routes under /api prefix
app.use('/api', categoryRoutes);
app.use('/api', accountRoutes);
app.use('/api', transactionRoutes);
app.use('/api', reportRoutes);
app.use('/api', budgetRoutes);
app.use('/api', alertRoutes);
app.use('/api', receiptRoutes);

// 404 handler for undefined routes
app.use(notFoundHandler);

// Global error handler (must be last)
app.use(errorHandler);

// Start server
const server = app.listen(PORT, () => {
  logger.info(`Node.js API server started on port ${PORT}`);
  logger.info(`Environment: ${process.env.NODE_ENV || 'development'}`);
});

// Graceful shutdown
process.on('SIGTERM', async () => {
  logger.info('SIGTERM signal received: closing HTTP server');
  server.close(async () => {
    logger.info('HTTP server closed');
    await closePool();
    process.exit(0);
  });
});

process.on('SIGINT', async () => {
  logger.info('SIGINT signal received: closing HTTP server');
  server.close(async () => {
    logger.info('HTTP server closed');
    await closePool();
    process.exit(0);
  });
});

// Handle uncaught exceptions
process.on('uncaughtException', (error) => {
  logger.error('Uncaught Exception', { error });
  process.exit(1);
});

// Handle unhandled promise rejections
process.on('unhandledRejection', (reason, promise) => {
  logger.error('Unhandled Rejection', { reason, promise });
  process.exit(1);
});

export default app;
