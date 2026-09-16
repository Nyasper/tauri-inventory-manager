export interface Category {
  id: number;
  name: string;
  color: string | null;
  icon: string | null;
}

export interface Presentation {
  id: number;
  product_id: number;
  name: string;
  factor: number;
}

export interface Product {
  archived: number;
  base_unit?: string;
  id: number;
  name: string;
  description: string | null;
  category_id: number | null;
  base_presentation_id: number | null;
  stock: number;
  created_at: string;
  updated_at: string;
  category_name?: string | null;
  category_color?: string | null;
  category_icon?: string | null;
}

export interface Supplier {
  archived?: number;
  id: number;
  name: string;
  contacts: string[];
  notes: string | null;
}

export type MovementType = "in" | "out";

export interface Movement {
  cancelled_at: string | null;
  cancellation_reason: string | null;
  kind: "regular" | "adjustment";
  reason: string | null;
  factor: number;
  id: number;
  product_id: number | null;
  type: MovementType;
  quantity: number;
  presentation_id: number | null;
  base_quantity: number;
  supplier_id: number | null;
  notes: string | null;
  created_at: string;
  product_name?: string;
  presentation_name?: string;
  supplier_name?: string;
}

export type DefaultView = "dashboard" | "products" | "movements" | "suppliers";

export interface AppSettings {
  stock_low_threshold: number;
  stock_medium_threshold: number;
  default_view: DefaultView;
  movements_page_size: number;
}

export type StockStatus = "out" | "low" | "medium" | "high";

export interface DuplicateRecord {
  id: number;
  archived: number;
}

export type AccentColor = "blue" | "violet" | "emerald" | "amber" | "rose" | "teal" | "slate";
