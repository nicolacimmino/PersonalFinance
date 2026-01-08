import { Request, Response } from 'express';
import * as service from './category.service';
import { CategoryDto } from './category.dto';
import { ApplicationCategory } from './category.model';

/**
 * Transform database model to DTO
 */
function toCategoryDto(category: ApplicationCategory): CategoryDto {
  return {
    code: category.code,
    color: category.color,
    discontinued: category.discontinued,
  };
}

/**
 * GET /api/categories
 * Get all categories
 */
export async function getCategories(
  _req: Request,
  res: Response
): Promise<void> {
  const categories = await service.getCategories();
  const dtos = categories.map(toCategoryDto);
  res.json(dtos);
}
