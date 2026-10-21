CREATE TABLE IF NOT EXISTS categories (
 id INTEGER PRIMARY KEY, name TEXT NOT NULL CHECK(length(trim(name)) > 0),
 name_key TEXT NOT NULL UNIQUE CHECK(length(name_key)>0), color TEXT, icon TEXT
);
CREATE TABLE IF NOT EXISTS suppliers (
 id INTEGER PRIMARY KEY, name TEXT NOT NULL CHECK(length(trim(name)) > 0),
 name_key TEXT NOT NULL UNIQUE CHECK(length(name_key)>0), contact TEXT, notes TEXT,
 archived INTEGER NOT NULL DEFAULT 0 CHECK(archived IN (0,1))
);
CREATE TABLE IF NOT EXISTS products (
 id INTEGER PRIMARY KEY, name TEXT NOT NULL CHECK(length(trim(name)) > 0),
 name_key TEXT NOT NULL UNIQUE CHECK(length(name_key)>0), description TEXT,
 category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
 default_supplier_id INTEGER REFERENCES suppliers(id) ON DELETE SET NULL,
 base_presentation_id INTEGER NOT NULL,
 stock REAL NOT NULL DEFAULT 0 CHECK(stock >= 0 AND stock <= 1000000000),
 stock_low_threshold REAL CHECK(stock_low_threshold IS NULL OR (stock_low_threshold >= 0 AND stock_low_threshold <= 1000000000)),
 stock_medium_threshold REAL CHECK(stock_medium_threshold IS NULL OR (stock_medium_threshold >= 0 AND stock_medium_threshold <= 1000000000)),
 ignore_stock_alerts INTEGER NOT NULL DEFAULT 0 CHECK(ignore_stock_alerts IN (0,1)),
 archived INTEGER NOT NULL DEFAULT 0 CHECK(archived IN (0,1)),
 created_at TEXT NOT NULL DEFAULT (datetime('now')), updated_at TEXT NOT NULL DEFAULT (datetime('now')),
 FOREIGN KEY(id, base_presentation_id) REFERENCES presentations(product_id,id) DEFERRABLE INITIALLY DEFERRED,
 CHECK(
  (stock_low_threshold IS NULL AND stock_medium_threshold IS NULL) OR
  (stock_low_threshold IS NOT NULL AND stock_medium_threshold IS NOT NULL AND stock_low_threshold < stock_medium_threshold)
 )
);
CREATE TABLE IF NOT EXISTS presentations (
 id INTEGER PRIMARY KEY, product_id INTEGER NOT NULL REFERENCES products(id),
 name TEXT NOT NULL CHECK(length(trim(name))>0), name_key TEXT NOT NULL CHECK(length(name_key)>0),
 factor REAL NOT NULL CHECK(factor > 0 AND factor <= 1000000000),
 archived INTEGER NOT NULL DEFAULT 0 CHECK(archived IN (0,1)),
 UNIQUE(product_id,name_key), UNIQUE(product_id,id)
);
CREATE TABLE IF NOT EXISTS movements (
 id INTEGER PRIMARY KEY, product_id INTEGER REFERENCES products(id) ON DELETE SET NULL,
 type TEXT NOT NULL CHECK(type IN ('in','out')),
 quantity REAL NOT NULL CHECK(quantity>0 AND quantity<=1000000000), presentation_id INTEGER,
 presentation_name TEXT NOT NULL, product_name TEXT NOT NULL,
 factor REAL NOT NULL CHECK(factor>0 AND factor<=1000000000),
 base_quantity REAL NOT NULL CHECK(base_quantity>0 AND base_quantity<=1000000000),
 supplier_id INTEGER REFERENCES suppliers(id) ON DELETE SET NULL, supplier_name TEXT,
 notes TEXT, reason TEXT, kind TEXT NOT NULL DEFAULT 'regular' CHECK(kind IN ('regular','adjustment')),
 created_at TEXT NOT NULL DEFAULT (datetime('now')), cancelled_at TEXT, cancellation_reason TEXT,
 FOREIGN KEY(product_id,presentation_id) REFERENCES presentations(product_id,id) ON DELETE SET NULL,
 CHECK(type='in' OR supplier_id IS NULL),
 CHECK(kind='regular' OR (reason IS NOT NULL AND length(trim(reason))>0)),
 CHECK((cancelled_at IS NULL AND cancellation_reason IS NULL) OR
       (cancelled_at IS NOT NULL AND cancellation_reason IS NOT NULL AND length(trim(cancellation_reason))>0))
);
CREATE INDEX IF NOT EXISTS movements_date ON movements(created_at DESC,id DESC);
CREATE INDEX IF NOT EXISTS movements_product ON movements(product_id,created_at DESC);
CREATE INDEX IF NOT EXISTS movements_supplier ON movements(supplier_id);
-- Se eliminó el índice de products(category_id): el filtrado por categoría se
-- hace en el cliente y ninguna consulta SQL lo aprovechaba.
DROP INDEX IF EXISTS products_category;
CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
INSERT OR IGNORE INTO settings(key,value) VALUES
 ('stock_low_threshold','5'),('stock_medium_threshold','20'),('default_view','dashboard'),
 ('delete_password','admin'),('movements_page_size','50');
