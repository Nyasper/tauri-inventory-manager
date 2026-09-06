export interface Category {
  id: number;
  name: string;
  color: string | null;
}

export interface Presentation {
  id: number;
  product_id: number;
  name: string;
  factor: number;
}

export interface Product {
  id: number;
  name: string;
  description: string | null;
  image: string | null;
  category_id: number | null;
  base_presentation_id: number | null;
  stock: number;
  created_at: string;
  updated_at: string;
  category_name?: string | null;
  category_color?: string | null;
  presentations?: Presentation[];
}

export interface Supplier {
  id: number;
  name: string;
  contact: string | null;
  notes: string | null;
}

export interface ResponsiblePerson {
  id: number;
  name: string;
  is_default: boolean;
}

export type MovementType = "in" | "out";

export interface Movement {
  id: number;
  product_id: number;
  type: MovementType;
  quantity: number;
  presentation_id: number;
  base_quantity: number;
  supplier_id: number | null;
  responsible_person_id: number | null;
  notes: string | null;
  created_at: string;
  product_name?: string;
  presentation_name?: string;
  supplier_name?: string;
  person_name?: string;
}

export type DefaultView =
  | "dashboard"
  | "products"
  | "movements"
  | "suppliers";

export interface AppSettings {
  stock_low_threshold: number;
  stock_medium_threshold: number;
  default_view: DefaultView;
}

export type StockStatus = "out" | "low" | "medium" | "high";

export type AccentColor =
  | "blue"
  | "violet"
  | "emerald"
  | "amber"
  | "rose"
  | "teal"
  | "slate";
