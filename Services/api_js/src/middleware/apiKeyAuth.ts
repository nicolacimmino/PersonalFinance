import { Request, Response, NextFunction } from 'express';
import { logger } from '../config/logger';

/**
 * Middleware to validate API key from x-api-key header
 * Matches the Rust ApiKey guard behavior
 */
export function apiKeyAuth(
  req: Request,
  res: Response,
  next: NextFunction
): void {
  const apiKey = req.headers['x-api-key'];
  const expectedApiKey = process.env.API_KEY;

  if (!expectedApiKey) {
    logger.error('API_KEY environment variable is not set');
    res.status(500).json({ error: 'Server configuration error' });
    return;
  }

  if (!apiKey) {
    logger.warn('Request without x-api-key header', {
      path: req.path,
      method: req.method,
    });
    res.status(401).json({ error: 'Unauthorized: Missing API key' });
    return;
  }

  if (apiKey !== expectedApiKey) {
    logger.warn('Request with invalid x-api-key', {
      path: req.path,
      method: req.method,
    });
    res.status(401).json({ error: 'Unauthorized: Invalid API key' });
    return;
  }

  // API key is valid, proceed to the next middleware/route
  next();
}
