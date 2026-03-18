import httpClient from './httpClient';
import type { Category } from '@/types';

/**
 * Category Service
 *
 * Handles all category-related API calls.
 */

/**
 * Get all categories
 */
export async function getCategories(): Promise<Category[]> {
  const response = await httpClient.get<Category[]>('/api/v2/categories/');
  return response;
}
