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
  const response = await httpClient.get<Category[]>('/api/categories/');
  return response;
}

/**
 * Get a single category by ID (if API supports it)
 */
export async function getCategory(id: string | number): Promise<Category> {
  const response = await httpClient.get<Category>(`/api/categories/${id}`);
  return response;
}

/**
 * Create a new category (if API supports it)
 */
export async function createCategory(category: Partial<Category>): Promise<Category> {
  const response = await httpClient.post<Category>('/api/categories/', category);
  return response;
}

/**
 * Update a category (if API supports it)
 */
export async function updateCategory(
  id: string | number,
  category: Partial<Category>
): Promise<Category> {
  const response = await httpClient.patch<Category>(`/api/categories/${id}`, category);
  return response;
}

/**
 * Delete a category (if API supports it)
 */
export async function deleteCategory(id: string | number): Promise<void> {
  await httpClient.delete(`/api/categories/${id}`);
}
