/**
 * Data Transfer Object for Alert responses
 */
export interface AlertDto {
  category: string;
  message: string;
  item: string;
  item_id: string;
  level: string;
}
