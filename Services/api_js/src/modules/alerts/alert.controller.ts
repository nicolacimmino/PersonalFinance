import { Request, Response } from 'express';
import * as service from './alert.service';
import { AlertDto } from './alert.dto';
import { Alert } from './alert.model';

/**
 * Transform database model to DTO
 */
function toAlertDto(alert: Alert): AlertDto {
  return {
    category: alert.category,
    message: alert.message,
    item: alert.item,
    item_id: alert.item_id,
    level: alert.level,
  };
}

/**
 * GET /api/alerts
 * Get all alerts
 */
export async function getAlerts(_req: Request, res: Response): Promise<void> {
  const alerts = await service.getAlerts();
  const dtos = alerts.map(toAlertDto);
  res.json(dtos);
}
