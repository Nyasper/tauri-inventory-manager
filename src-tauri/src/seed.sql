INSERT INTO categories(id,name,name_key,color,icon) VALUES
  (1,'Bebidas','bebidas','#3b82f6','cup-soda'),
  (2,'Alimentos','alimentos','#22c55e','apple'),
  (3,'Limpieza','limpieza','#8b5cf6','spray-can'),
  (4,'Higiene','higiene','#14b8a6','soap-dispenser-droplet');

INSERT INTO suppliers(id,name,name_key,contact,notes) VALUES
  (1,'Distribuidora Norte','distribuidora norte','["contacto@norte.example","099 111 222"]','Entrega los lunes'),
  (2,'Mayorista Central','mayorista central','["ventas@central.example","2900 1234"]','Pedidos desde 12 unidades'),
  (3,'Limpiezas del Valle','limpiezas del valle','["099 333 444","pedidos@valle.example"]','Especialistas en productos de limpieza');

INSERT INTO responsible_persons(id,name,name_key,is_default) VALUES
  (1,'Ana Pérez','ana pérez',1),
  (2,'Carlos Gómez','carlos gómez',0),
  (3,'Lucía Martín','lucía martín',0);

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

INSERT INTO movements(product_id,type,quantity,presentation_id,presentation_name,product_name,factor,base_quantity,supplier_id,supplier_name,responsible_person_id,person_name,notes,created_at) VALUES
  (1,'in',120,1,'Botella','Agua Mineral 500ml',1,120,1,'Distribuidora Norte',1,'Ana Pérez','Compra inicial',datetime('now','-10 days')),
  (1,'out',18,1,'Botella','Agua Mineral 500ml',1,18,NULL,NULL,2,'Carlos Gómez','Consumo interno',datetime('now','-1 day')),
  (2,'in',60,3,'Paquete','Café Molido 250g',1,60,2,'Mayorista Central',1,'Ana Pérez','Compra inicial',datetime('now','-9 days')),
  (2,'out',15,3,'Paquete','Café Molido 250g',1,15,NULL,NULL,3,'Lucía Martín','Consumo de oficina',datetime('now','-2 days')),
  (3,'in',48,5,'Botella','Detergente 1L',1,48,3,'Limpiezas del Valle',1,'Ana Pérez','Compra inicial',datetime('now','-8 days')),
  (3,'out',18,5,'Botella','Detergente 1L',1,18,NULL,NULL,2,'Carlos Gómez','Uso de limpieza',datetime('now','-3 days')),
  (4,'in',24,7,'Unidad','Jabón de Manos',1,24,3,'Limpiezas del Valle',1,'Ana Pérez','Compra inicial',datetime('now','-7 days')),
  (4,'out',6,7,'Unidad','Jabón de Manos',1,6,NULL,NULL,3,'Lucía Martín','Reposición de baños',datetime('now','-2 days')),
  (5,'in',48,9,'Paquete','Galletas Variadas',1,48,2,'Mayorista Central',1,'Ana Pérez','Compra inicial',datetime('now','-6 days')),
  (5,'out',12,9,'Paquete','Galletas Variadas',1,12,NULL,NULL,2,'Carlos Gómez','Reunión de equipo',datetime('now','-1 day')),
  (6,'in',30,11,'Botella','Shampoo 400ml',1,30,3,'Limpiezas del Valle',1,'Ana Pérez','Compra inicial',datetime('now','-5 days')),
  (6,'out',6,11,'Botella','Shampoo 400ml',1,6,NULL,NULL,3,'Lucía Martín','Entrega interna',datetime('now'));
