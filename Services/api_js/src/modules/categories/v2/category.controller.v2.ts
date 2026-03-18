import { Request, Response } from 'express';
import * as service from '../category.service';
import { CategoryDtoV2 } from './category.dto.v2';
import { ApplicationCategory } from '../category.model';

/**
 * Transform database model to V2 DTO (camelCase)
 * Note: Categories already have clean naming, so transformation is identical to v1
 */
function toCategoryDtoV2(category: ApplicationCategory): CategoryDtoV2 {
  return {
    code: category.code,
    color: category.color,
    discontinued: category.discontinued,
  };
}

/**
 * GET /api/v2/categories
 * Get all categories
 */
export async function getCategories(
  _req: Request,
  res: Response
): Promise<void> {
  const categories = await service.getCategories();
  const dtos = categories.map(toCategoryDtoV2);
  res.json(dtos);
}
