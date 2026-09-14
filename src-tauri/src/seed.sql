INSERT INTO categories(id,name,name_key,color,icon) VALUES
  (1,'Bebidas','bebidas','#3b82f6','cup-soda'),
  (2,'Alimentos','alimentos','#22c55e','apple'),
  (3,'Limpieza','limpieza','#8b5cf6','spray-can'),
  (4,'Higiene','higiene','#14b8a6','soap-dispenser-droplet');

INSERT INTO suppliers(id,name,name_key,contact,notes) VALUES
  (1,'Distribuidora Norte','distribuidora norte','["contacto@norte.example","099 111 222"]','Entrega los lunes'),
  (2,'Mayorista Central','mayorista central','["ventas@central.example","2900 1234"]','Pedidos desde 12 unidades'),
  (3,'Limpiezas del Valle','limpiezas del valle','["099 333 444","pedidos@valle.example"]','Especialistas en productos de limpieza');

INSERT INTO products(id,name,name_key,description,category_id,base_presentation_id,stock,created_at,updated_at) VALUES
  (1,'Agua Mineral 500ml','agua mineral 500ml','Agua sin gas en botella',1,1,102,datetime('now','-10 days'),datetime('now','-1 day')),
  (2,'Café Molido 250g','café molido 250g','Café tostado de origen',2,3,45,datetime('now','-9 days'),datetime('now','-2 days')),
  (3,'Detergente 1L','detergente 1l','Detergente líquido concentrado',3,5,30,datetime('now','-8 days'),datetime('now','-3 days')),
  (4,'Jabón de Manos','jabón de manos','Jabón líquido antibacterial',4,7,18,datetime('now','-7 days'),datetime('now','-2 days')),
  (5,'Galletas Variadas','galletas variadas','Paquete surtido',2,9,36,datetime('now','-6 days'),datetime('now','-1 day')),
  (6,'Shampoo 400ml','shampoo 400ml','Shampoo hidratante',4,11,24,datetime('now','-5 days'),datetime('now'));

INSERT INTO presentations(id,product_id,name,name_key,factor) VALUES
  (1,1,'Botella','botella',1),(2,1,'Caja x6','caja x6',6),
  (3,2,'Paquete','paquete',1),(4,2,'Caja x12','caja x12',12),
  (5,3,'Botella','botella',1),(6,3,'Caja x6','caja x6',6),
  (7,4,'Unidad','unidad',1),(8,4,'Caja x12','caja x12',12),
  (9,5,'Paquete','paquete',1),(10,5,'Caja x12','caja x12',12),
  (11,6,'Botella','botella',1),(12,6,'Caja x6','caja x6',6);

INSERT INTO movements(product_id,type,quantity,presentation_id,presentation_name,product_name,factor,base_quantity,supplier_id,supplier_name,notes,created_at) VALUES
  (1,'in',120,1,'Botella','Agua Mineral 500ml',1,120,1,'Distribuidora Norte','Compra inicial',datetime('now','-10 days')),
  (1,'out',18,1,'Botella','Agua Mineral 500ml',1,18,NULL,NULL,'Consumo interno',datetime('now','-1 day')),
  (2,'in',60,3,'Paquete','Café Molido 250g',1,60,2,'Mayorista Central','Compra inicial',datetime('now','-9 days')),
  (2,'out',15,3,'Paquete','Café Molido 250g',1,15,NULL,NULL,'Consumo de oficina',datetime('now','-2 days')),
  (3,'in',48,5,'Botella','Detergente 1L',1,48,3,'Limpiezas del Valle','Compra inicial',datetime('now','-8 days')),
  (3,'out',18,5,'Botella','Detergente 1L',1,18,NULL,NULL,'Uso de limpieza',datetime('now','-3 days')),
  (4,'in',24,7,'Unidad','Jabón de Manos',1,24,3,'Limpiezas del Valle','Compra inicial',datetime('now','-7 days')),
  (4,'out',6,7,'Unidad','Jabón de Manos',1,6,NULL,NULL,'Reposición de baños',datetime('now','-2 days')),
  (5,'in',48,9,'Paquete','Galletas Variadas',1,48,2,'Mayorista Central','Compra inicial',datetime('now','-6 days')),
  (5,'out',12,9,'Paquete','Galletas Variadas',1,12,NULL,NULL,'Reunión de equipo',datetime('now','-1 day')),
  (6,'in',30,11,'Botella','Shampoo 400ml',1,30,3,'Limpiezas del Valle','Compra inicial',datetime('now','-5 days')),
  (6,'out',6,11,'Botella','Shampoo 400ml',1,6,NULL,NULL,'Entrega interna',datetime('now'));

-- Datos adicionales para poblar la aplicación con un inventario más realista.
INSERT INTO categories(id,name,name_key,color,icon) VALUES
  (5,'Oficina','oficina','#f59e0b','briefcase-business'),
  (6,'Tecnología','tecnología','#ef4444','monitor'),
  (7,'Snacks','snacks','#ec4899','cookie'),
  (8,'Cocina','cocina','#0ea5e9','utensils');

INSERT INTO suppliers(id,name,name_key,contact,notes) VALUES
  (4,'Alimentos del Sur','alimentos del sur','["ventas@sur.example","098 444 555"]','Envíos a todo el país'),
  (5,'Higiene Total','higiene total','["099 666 777"]','Línea institucional'),
  (6,'Papelería 360','papelería 360','["contacto@papeleria360.example","2901 2233"]','Precios por volumen'),
  (7,'Tecno Oficina','tecno oficina','["ventas@tecnooficina.example","2902 3344"]','Accesorios y consumibles'),
  (8,'Café & Más','café & más','["pedidos@cafeymas.example","097 888 999"]','Tostado semanal'),
  (9,'Importadora del Plata','importadora del plata','["compras@plata.example"]','Importaciones mensuales'),
  (10,'Lácteos del Prado','lácteos del prado','["099 123 987"]','Cadena de frío');

INSERT INTO products(id,name,name_key,description,category_id,base_presentation_id,stock,created_at,updated_at) VALUES
  (7,'Arroz Blanco 1kg','arroz blanco 1kg','Arroz largo fino',2,13,68,datetime('now','-20 days'),datetime('now','-1 day')),
  (8,'Aceite de Girasol 900ml','aceite de girasol 900ml','Aceite comestible',2,15,12,datetime('now','-19 days'),datetime('now','-2 days')),
  (9,'Yerba Mate 1kg','yerba mate 1kg','Yerba mate suave',2,17,4,datetime('now','-18 days'),datetime('now')),
  (10,'Leche Entera 1L','leche entera 1l','Leche larga vida',8,19,31,datetime('now','-17 days'),datetime('now','-1 day')),
  (11,'Atún en Lata 170g','atún en lata 170g','Atún al natural',2,21,0,datetime('now','-16 days'),datetime('now','-3 days')),
  (12,'Papel Higiénico x4','papel higiénico x4','Pack de cuatro rollos',4,23,22,datetime('now','-15 days'),datetime('now','-1 day')),
  (13,'Toallas de Papel','toallas de papel','Rollo absorbente de cocina',3,25,7,datetime('now','-14 days'),datetime('now')),
  (14,'Lavandina 1L','lavandina 1l','Hipoclorito de sodio',3,27,55,datetime('now','-13 days'),datetime('now','-2 days')),
  (15,'Esponja Multiuso','esponja multiuso','Esponja abrasiva',3,29,9,datetime('now','-12 days'),datetime('now','-1 day')),
  (16,'Alcohol en Gel 500ml','alcohol en gel 500ml','Sanitizante de manos',4,31,16,datetime('now','-11 days'),datetime('now')),
  (17,'Cuaderno A4','cuaderno a4','Cuaderno tapa dura 100 hojas',5,33,43,datetime('now','-10 days'),datetime('now','-1 day')),
  (18,'Resma A4 500 hojas','resma a4 500 hojas','Papel obra 75g',5,35,18,datetime('now','-9 days'),datetime('now','-2 days')),
  (19,'Lapicera Azul','lapicera azul','Bolígrafo tinta azul',5,37,126,datetime('now','-8 days'),datetime('now')),
  (20,'Marcador Permanente','marcador permanente','Punta gruesa negro',5,39,3,datetime('now','-7 days'),datetime('now','-1 day')),
  (21,'Memoria USB 32GB','memoria usb 32gb','Unidad flash USB 3.0',6,41,14,datetime('now','-6 days'),datetime('now','-1 day')),
  (22,'Cable HDMI 2m','cable hdmi 2m','Cable HDMI alta velocidad',6,43,8,datetime('now','-5 days'),datetime('now')),
  (23,'Pilas AA x4','pilas aa x4','Pilas alcalinas',6,45,27,datetime('now','-4 days'),datetime('now','-1 day')),
  (24,'Barra de Cereal','barra de cereal','Snack de avena y miel',7,47,64,datetime('now','-3 days'),datetime('now')),
  (25,'Galletas de Agua','galletas de agua','Paquete de galletas saladas',7,49,11,datetime('now','-2 days'),datetime('now')),
  (26,'Azúcar 1kg','azúcar 1kg','Azúcar blanca refinada',8,51,38,datetime('now','-1 day'),datetime('now'));

INSERT INTO presentations(id,product_id,name,name_key,factor) VALUES
  (13,7,'Paquete','paquete',1),(14,7,'Bulto x10','bulto x10',10),
  (15,8,'Botella','botella',1),(16,8,'Caja x12','caja x12',12),
  (17,9,'Paquete','paquete',1),(18,9,'Caja x10','caja x10',10),
  (19,10,'Caja','caja',1),(20,10,'Pack x12','pack x12',12),
  (21,11,'Lata','lata',1),(22,11,'Caja x24','caja x24',24),
  (23,12,'Pack','pack',1),(24,12,'Bulto x8','bulto x8',8),
  (25,13,'Rollo','rollo',1),(26,13,'Pack x6','pack x6',6),
  (27,14,'Botella','botella',1),(28,14,'Caja x12','caja x12',12),
  (29,15,'Unidad','unidad',1),(30,15,'Pack x10','pack x10',10),
  (31,16,'Botella','botella',1),(32,16,'Caja x12','caja x12',12),
  (33,17,'Unidad','unidad',1),(34,17,'Caja x20','caja x20',20),
  (35,18,'Resma','resma',1),(36,18,'Pallet x5','pallet x5',5),
  (37,19,'Unidad','unidad',1),(38,19,'Caja x50','caja x50',50),
  (39,20,'Unidad','unidad',1),(40,20,'Caja x12','caja x12',12),
  (41,21,'Unidad','unidad',1),(42,21,'Caja x10','caja x10',10),
  (43,22,'Unidad','unidad',1),(44,22,'Pack x5','pack x5',5),
  (45,23,'Blister','blister',1),(46,23,'Caja x20','caja x20',20),
  (47,24,'Unidad','unidad',1),(48,24,'Caja x24','caja x24',24),
  (49,25,'Paquete','paquete',1),(50,25,'Caja x20','caja x20',20),
  (51,26,'Paquete','paquete',1),(52,26,'Bulto x10','bulto x10',10);

INSERT INTO movements(product_id,type,quantity,presentation_id,presentation_name,product_name,factor,base_quantity,supplier_id,supplier_name,notes,created_at) VALUES
  (7,'in',80,13,'Paquete','Arroz Blanco 1kg',1,80,4,'Alimentos del Sur','Reposición mensual',datetime('now','-20 days')),(7,'out',12,13,'Paquete','Arroz Blanco 1kg',1,12,NULL,NULL,'Consumo comedor',datetime('now','-1 day')),
  (8,'in',24,15,'Botella','Aceite de Girasol 900ml',1,24,4,'Alimentos del Sur','Compra mayorista',datetime('now','-19 days')),(8,'out',12,15,'Botella','Aceite de Girasol 900ml',1,12,NULL,NULL,'Cocina',datetime('now','-2 days')),
  (9,'in',10,18,'Caja x10','Yerba Mate 1kg',10,100,8,'Café & Más','Compra por caja',datetime('now','-18 days')),(9,'out',96,17,'Paquete','Yerba Mate 1kg',1,96,NULL,NULL,'Consumo oficina',datetime('now')),
  (10,'in',3,20,'Pack x12','Leche Entera 1L',12,36,10,'Lácteos del Prado','Entrega refrigerada',datetime('now','-17 days')),(10,'out',5,19,'Caja','Leche Entera 1L',1,5,NULL,NULL,'Desayuno',datetime('now','-1 day')),
  (11,'in',24,22,'Caja x24','Atún en Lata 170g',24,576,4,'Alimentos del Sur','Compra inicial',datetime('now','-16 days')),(11,'out',576,21,'Lata','Atún en Lata 170g',1,576,NULL,NULL,'Donación',datetime('now','-3 days')),
  (12,'in',4,24,'Bulto x8','Papel Higiénico x4',8,32,5,'Higiene Total','Stock de baños',datetime('now','-15 days')),(12,'out',10,23,'Pack','Papel Higiénico x4',1,10,NULL,NULL,'Reposición',datetime('now','-1 day')),
  (13,'in',12,26,'Pack x6','Toallas de Papel',6,72,5,'Higiene Total','Compra por volumen',datetime('now','-14 days')),(13,'out',65,25,'Rollo','Toallas de Papel',1,65,NULL,NULL,'Cocina',datetime('now')),
  (14,'in',60,27,'Botella','Lavandina 1L',1,60,3,'Limpiezas del Valle','Reposición',datetime('now','-13 days')),(14,'out',5,27,'Botella','Lavandina 1L',1,5,NULL,NULL,'Limpieza general',datetime('now','-2 days')),
  (15,'in',20,30,'Pack x10','Esponja Multiuso',10,200,5,'Higiene Total','Compra por pack',datetime('now','-12 days')),(15,'out',191,29,'Unidad','Esponja Multiuso',1,191,NULL,NULL,'Uso diario',datetime('now','-1 day')),
  (16,'in',24,32,'Caja x12','Alcohol en Gel 500ml',12,288,5,'Higiene Total','Prevención',datetime('now','-11 days')),(16,'out',272,31,'Botella','Alcohol en Gel 500ml',1,272,NULL,NULL,'Distribución',datetime('now')),
  (17,'in',60,33,'Unidad','Cuaderno A4',1,60,6,'Papelería 360','Material de oficina',datetime('now','-10 days')),(17,'out',17,33,'Unidad','Cuaderno A4',1,17,NULL,NULL,'Entrega a equipo',datetime('now','-1 day')),
  (18,'in',20,36,'Pallet x5','Resma A4 500 hojas',5,100,6,'Papelería 360','Compra trimestral',datetime('now','-9 days')),(18,'out',82,35,'Resma','Resma A4 500 hojas',1,82,NULL,NULL,'Impresiones',datetime('now','-2 days')),
  (19,'in',150,38,'Caja x50','Lapicera Azul',50,7500,6,'Papelería 360','Compra por caja',datetime('now','-8 days')),(19,'out',7374,37,'Unidad','Lapicera Azul',1,7374,NULL,NULL,'Consumo general',datetime('now')),
  (20,'in',24,40,'Caja x12','Marcador Permanente',12,288,6,'Papelería 360','Material de oficina',datetime('now','-7 days')),(20,'out',285,39,'Unidad','Marcador Permanente',1,285,NULL,NULL,'Uso en depósitos',datetime('now','-1 day')),
  (21,'in',20,42,'Caja x10','Memoria USB 32GB',10,200,7,'Tecno Oficina','Accesorios',datetime('now','-6 days')),(21,'out',186,41,'Unidad','Memoria USB 32GB',1,186,NULL,NULL,'Asignaciones',datetime('now','-1 day')),
  (22,'in',10,44,'Pack x5','Cable HDMI 2m',5,50,7,'Tecno Oficina','Accesorios audiovisuales',datetime('now','-5 days')),(22,'out',42,43,'Unidad','Cable HDMI 2m',1,42,NULL,NULL,'Salas de reunión',datetime('now')),
  (23,'in',30,46,'Caja x20','Pilas AA x4',20,600,9,'Importadora del Plata','Compra de baterías',datetime('now','-4 days')),(23,'out',573,45,'Blister','Pilas AA x4',1,573,NULL,NULL,'Periféricos',datetime('now','-1 day')),
  (24,'in',72,48,'Caja x24','Barra de Cereal',24,1728,4,'Alimentos del Sur','Snack saludable',datetime('now','-3 days')),(24,'out',1664,47,'Unidad','Barra de Cereal',1,1664,NULL,NULL,'Consumo oficina',datetime('now')),
  (25,'in',20,50,'Caja x20','Galletas de Agua',20,400,4,'Alimentos del Sur','Compra mensual',datetime('now','-2 days')),(25,'out',389,49,'Paquete','Galletas de Agua',1,389,NULL,NULL,'Reunión',datetime('now')),
  (26,'in',5,52,'Bulto x10','Azúcar 1kg',10,50,4,'Alimentos del Sur','Abastecimiento cocina',datetime('now','-1 day')),(26,'out',12,51,'Paquete','Azúcar 1kg',1,12,NULL,NULL,'Cafetería',datetime('now'));
