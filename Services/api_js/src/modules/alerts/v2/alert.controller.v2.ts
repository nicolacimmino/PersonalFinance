import { Request, Response } from 'express';
import * as service from '../alert.service';
import { AlertDtoV2 } from './alert.dto.v2';
import { Alert } from '../alert.model';

/**
 * Transform database model to V2 DTO (camelCase)
 */
function toAlertDtoV2(alert: Alert): AlertDtoV2 {
  return {
    category: alert.category,
    message: alert.message,
    item: alert.item,
    itemId: alert.item_id,
    level: alert.level,
  };
}

/**
 * GET /api/v2/alerts
 * Get all alerts
 */
export async function getAlerts(_req: Request, res: Response): Promise<void> {
  const alerts = await service.getAlerts();
  const dtos = alerts.map(toAlertDtoV2);
  res.json(dtos);
}
