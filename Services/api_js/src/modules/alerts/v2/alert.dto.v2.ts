/**
 * V2 Data Transfer Object for Alert responses (camelCase)
 */
export interface AlertDtoV2 {
  category: string;
  message: string;
  item: string;
  itemId: string;
  level: string;
}
