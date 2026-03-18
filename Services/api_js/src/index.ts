import 'express-async-errors';
import express, { Application } from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import { logger } from './config/logger';
import { closePool } from './config/database';
import { errorHandler, notFoundHandler } from './middleware/errorHandler';

// Import versioned route bundles
import v1Routes from './routes/v1.routes';
import v2Routes from './routes/v2.routes';

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

// Mount versioned API routes
app.use('/api/v1', v1Routes);  // V1 at /api/v1/*
app.use('/api/v2', v2Routes);  // V2 at /api/v2/*

// For backward compatibility, mount v1 at /api/* as well
app.use('/api', v1Routes);     // Legacy /api/* → v1

// 404 handler for undefined routes
app.use(notFoundHandler);

// Global error handler (must be last)
app.use(errorHandler);

// Start server
const server = app.listen(PORT, () => {
  logger.info(`Node.js API server started on port ${PORT}`);
  logger.info(`Environment: ${process.env.NODE_ENV || 'development'}`);
  logger.info('API v1: /api/v1/* and /api/* (legacy)');
  logger.info('API v2: /api/v2/* (camelCase)');
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
