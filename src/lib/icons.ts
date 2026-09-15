import type { Component } from "svelte";
import type { LucideProps } from "@lucide/svelte";
import {
  Activity,
  Apple,
  Archive,
  ArrowDownToLine,
  ArrowLeftRight,
  ArrowUpFromLine,
  Bath,
  BatteryCharging,
  Beer,
  Bike,
  Book,
  BookOpen,
  Box,
  Boxes,
  Briefcase,
  BriefcaseBusiness,
  Broom,
  BroomSparkles,
  Brush,
  Candy,
  ChefHat,
  ChevronDown,
  ChevronUp,
  ClipboardList,
  Coffee,
  Container,
  Cookie,
  CreditCard,
  CupSoda,
  Droplets,
  Dumbbell,
  Egg,
  File,
  Flame,
  Folder,
  Footprints,
  Gamepad,
  Gift,
  GlassWater,
  Headphones,
  HeartPulse,
  IceCreamBowl,
  KeyRound,
  Laptop,
  Layers,
  LayoutDashboard,
  LayoutGrid,
  Lightbulb,
  List,
  Lollipop,
  Milk,
  MopSparkles,
  Moon,
  Package,
  PackageOpen,
  PaintBucket,
  Palette,
  Pencil,
  PenTool,
  PencilRuler,
  Pill,
  Plug,
  Plus,
  Printer,
  RefreshCw,
  Salad,
  Scan,
  Scissors,
  Search,
  Settings,
  Shirt,
  ShoppingBasket,
  ShoppingCart,
  ShowerHead,
  Smartphone,
  Sparkles,
  SprayCan,
  SoapDispenserDroplet,
  Stethoscope,
  Store,
  Sun,
  Syringe,
  Tag,
  Trash,
  TriangleAlert,
  Truck,
  Toilet,
  User,
  Users,
  Utensils,
  Warehouse,
  WashingMachine,
  Watch,
  Wine,
  Wrench,
  X,
  Zap,
} from "@lucide/svelte";

export type IconName =
  | "box"
  | "boxes"
  | "package"
  | "package-open"
  | "archive"
  | "container"
  | "warehouse"
  | "store"
  | "cup-soda"
  | "glass-water"
  | "coffee"
  | "milk"
  | "wine"
  | "beer"
  | "apple"
  | "egg"
  | "cookie"
  | "candy"
  | "lollipop"
  | "salad"
  | "utensils"
  | "chef-hat"
  | "ice-cream-bowl"
  | "spray-can"
  | "broom"
  | "broom-sparkles"
  | "mop-sparkles"
  | "soap-dispenser-droplet"
  | "toilet"
  | "brush"
  | "paint-bucket"
  | "droplets"
  | "bath"
  | "shower-head"
  | "washing-machine"
  | "shirt"
  | "gift"
  | "pill"
  | "syringe"
  | "scissors"
  | "search"
  | "plus"
  | "pencil"
  | "trash"
  | "x"
  | "layout-grid"
  | "list"
  | "sun"
  | "moon"
  | "settings"
  | "arrow-left-right"
  | "arrow-down-to-line"
  | "arrow-up-from-line"
  | "truck"
  | "user"
  | "users"
  | "tag"
  | "layout-dashboard"
  | "triangle-alert"
  | "activity"
  | "chevron-up"
  | "chevron-down"
  | "clipboard-list"
  | "shopping-basket"
  | "shopping-cart"
  | "briefcase"
  | "briefcase-business"
  | "refresh-cw"
  | "palette"
  | "sparkles"
  | "layers"
  | "scan"
  | "lightbulb"
  | "flame"
  | "zap"
  | "plug"
  | "battery-charging"
  | "laptop"
  | "smartphone"
  | "watch"
  | "headphones"
  | "printer"
  | "gamepad"
  | "dumbbell"
  | "bike"
  | "footprints"
  | "book"
  | "book-open"
  | "folder"
  | "file"
  | "pencil-ruler"
  | "pen-tool"
  | "wrench"
  | "key-round"
  | "credit-card"
  | "heart-pulse"
  | "stethoscope";

type LucideIcon = Component<LucideProps>;

export const ICONS: Record<IconName, LucideIcon> = {
  box: Box,
  boxes: Boxes,
  package: Package,
  "package-open": PackageOpen,
  archive: Archive,
  container: Container,
  warehouse: Warehouse,
  store: Store,
  "cup-soda": CupSoda,
  "glass-water": GlassWater,
  coffee: Coffee,
  milk: Milk,
  wine: Wine,
  beer: Beer,
  apple: Apple,
  egg: Egg,
  cookie: Cookie,
  candy: Candy,
  lollipop: Lollipop,
  salad: Salad,
  utensils: Utensils,
  "chef-hat": ChefHat,
  "ice-cream-bowl": IceCreamBowl,
  "spray-can": SprayCan,
  broom: Broom,
  "broom-sparkles": BroomSparkles,
  "mop-sparkles": MopSparkles,
  "soap-dispenser-droplet": SoapDispenserDroplet,
  toilet: Toilet,
  brush: Brush,
  "paint-bucket": PaintBucket,
  droplets: Droplets,
  bath: Bath,
  "shower-head": ShowerHead,
  "washing-machine": WashingMachine,
  shirt: Shirt,
  gift: Gift,
  pill: Pill,
  syringe: Syringe,
  scissors: Scissors,
  search: Search,
  plus: Plus,
  pencil: Pencil,
  trash: Trash,
  x: X,
  "layout-grid": LayoutGrid,
  list: List,
  sun: Sun,
  moon: Moon,
  settings: Settings,
  "arrow-left-right": ArrowLeftRight,
  "arrow-down-to-line": ArrowDownToLine,
  "arrow-up-from-line": ArrowUpFromLine,
  truck: Truck,
  user: User,
  users: Users,
  tag: Tag,
  "layout-dashboard": LayoutDashboard,
  "triangle-alert": TriangleAlert,
  activity: Activity,
  "chevron-up": ChevronUp,
  "chevron-down": ChevronDown,
  "clipboard-list": ClipboardList,
  "shopping-basket": ShoppingBasket,
  "shopping-cart": ShoppingCart,
  briefcase: Briefcase,
  "briefcase-business": BriefcaseBusiness,
  "refresh-cw": RefreshCw,
  palette: Palette,
  sparkles: Sparkles,
  layers: Layers,
  scan: Scan,
  lightbulb: Lightbulb,
  flame: Flame,
  zap: Zap,
  plug: Plug,
  "battery-charging": BatteryCharging,
  laptop: Laptop,
  smartphone: Smartphone,
  watch: Watch,
  headphones: Headphones,
  printer: Printer,
  gamepad: Gamepad,
  dumbbell: Dumbbell,
  bike: Bike,
  footprints: Footprints,
  book: Book,
  "book-open": BookOpen,
  folder: Folder,
  file: File,
  "pencil-ruler": PencilRuler,
  "pen-tool": PenTool,
  wrench: Wrench,
  "key-round": KeyRound,
  "credit-card": CreditCard,
  "heart-pulse": HeartPulse,
  stethoscope: Stethoscope,
};

export interface IconOption {
  value: IconName;
  label: string;
  group: string;
}

export const ICON_OPTIONS: IconOption[] = [
  { value: "cup-soda", label: "Vaso refresco", group: "Bebidas" },
  { value: "glass-water", label: "Vaso agua", group: "Bebidas" },
  { value: "coffee", label: "Café", group: "Bebidas" },
  { value: "milk", label: "Leche", group: "Bebidas" },
  { value: "wine", label: "Vino", group: "Bebidas" },
  { value: "beer", label: "Cerveza", group: "Bebidas" },
  { value: "apple", label: "Fruta", group: "Alimentos" },
  { value: "egg", label: "Huevos", group: "Alimentos" },
  { value: "cookie", label: "Galletas", group: "Alimentos" },
  { value: "candy", label: "Dulces", group: "Alimentos" },
  { value: "lollipop", label: "Caramelo", group: "Alimentos" },
  { value: "salad", label: "Ensalada", group: "Alimentos" },
  { value: "utensils", label: "Cubiertos", group: "Alimentos" },
  { value: "chef-hat", label: "Cocina", group: "Alimentos" },
  { value: "ice-cream-bowl", label: "Helado", group: "Alimentos" },
  { value: "spray-can", label: "Aerosol", group: "Limpieza" },
  { value: "broom", label: "Escoba", group: "Limpieza" },
  { value: "broom-sparkles", label: "Barrido", group: "Limpieza" },
  { value: "mop-sparkles", label: "Mopa", group: "Limpieza" },
  {
    value: "soap-dispenser-droplet",
    label: "Dispensador de jabón",
    group: "Limpieza",
  },
  { value: "toilet", label: "Inodoro", group: "Limpieza" },
  { value: "brush", label: "Cepillo", group: "Limpieza" },
  { value: "paint-bucket", label: "Balde", group: "Limpieza" },
  { value: "droplets", label: "Gotas", group: "Limpieza" },
  { value: "bath", label: "Baño", group: "Higiene" },
  { value: "shower-head", label: "Ducha", group: "Higiene" },
  { value: "washing-machine", label: "Lavadora", group: "Higiene" },
  { value: "shirt", label: "Ropa", group: "Higiene" },
  { value: "gift", label: "Regalo", group: "Varios" },
  { value: "pill", label: "Pastillas", group: "Varios" },
  { value: "syringe", label: "Inyección", group: "Varios" },
  { value: "scissors", label: "Tijeras", group: "Varios" },
  { value: "box", label: "Caja", group: "Contenedores" },
  { value: "boxes", label: "Cajas", group: "Contenedores" },
  { value: "package", label: "Paquete", group: "Contenedores" },
  { value: "package-open", label: "Paquete abierto", group: "Contenedores" },
  { value: "archive", label: "Archivo", group: "Contenedores" },
  { value: "container", label: "Contenedor", group: "Contenedores" },
  { value: "warehouse", label: "Almacén", group: "Contenedores" },
  { value: "store", label: "Tienda", group: "Contenedores" },
  { value: "shopping-cart", label: "Carrito", group: "Varios" },
  { value: "shopping-basket", label: "Canasta", group: "Varios" },
  { value: "lightbulb", label: "Bombilla", group: "Varios" },
  { value: "battery-charging", label: "Batería", group: "Varios" },
  { value: "printer", label: "Impresora", group: "Varios" },
  { value: "laptop", label: "Portátil", group: "Varios" },
  { value: "headphones", label: "Audífonos", group: "Varios" },
  { value: "smartphone", label: "Teléfono", group: "Varios" },
  { value: "book", label: "Libro", group: "Varios" },
  { value: "wrench", label: "Llave inglesa", group: "Varios" },
  { value: "heart-pulse", label: "Salud", group: "Varios" },
  { value: "tag", label: "Etiqueta", group: "Varios" },
];

const CATEGORY_ICON_FALLBACKS: Record<string, IconName> = {
  bebidas: "cup-soda",
  alimentos: "apple",
  limpieza: "spray-can",
  higiene: "droplets",
  otros: "boxes",
};

export function iconNameFor(value: string | null | undefined): IconName {
  if (!value) return "box";
  const key = value.toLowerCase();
  if (key in ICONS) return key as IconName;
  for (const [label, icon] of Object.entries(CATEGORY_ICON_FALLBACKS)) {
    if (key.includes(label)) return icon;
  }
  return "box";
}
