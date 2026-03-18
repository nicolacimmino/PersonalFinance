/**
 * V2 Data Transfer Object for Category responses (camelCase)
 * Note: Categories already use clean naming, so v2 is identical to v1
 */
export interface CategoryDtoV2 {
  code: string;
  color: string;
  discontinued: string;
}
