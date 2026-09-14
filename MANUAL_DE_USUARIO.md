# Manual de Usuario — Inventory Manager

Bienvenido al **Manual de Usuario oficial de Inventory Manager**, la solución de escritorio moderna, ágil y segura para el control de stock, gestión de productos, presentaciones con factores de conversión, trazabilidad de movimientos y administración de proveedores.

---

## Índice de Contenidos

1. [Introducción y Arquitectura General](#1-introducción-y-arquitectura-general)
2. [Instalación y Primer Inicio](#2-instalación-y-primer-inicio)
   - [Requisitos del Sistema](#requisitos-del-sistema)
   - [Instalación en Windows y Linux](#instalación-en-windows-y-linux)
   - [Ubicación de los Datos y Base de Datos Local](#ubicación-de-los-datos-y-base-de-datos-local)
   - [Datos de Demostración (Opcional)](#datos-de-demostración-opcional)
3. [Estructura de la Interfaz y Navegación](#3-estructura-de-la-interfaz-y-navegación)
   - [Barra Lateral (Sidebar)](#barra-lateral-sidebar)
   - [Cabecera (Header) y Controles Globales](#cabecera-header-y-controles-globales)
   - [Modo Oscuro / Claro](#modo-oscuro--claro)
   - [Notificaciones del Sistema (Toasts) y Modales](#notificaciones-del-sistema-toasts-y-modales)
   - [Atajos de Teclado](#atajos-de-teclado)
4. [Panel de Control (Dashboard)](#4-panel-de-control-dashboard)
   - [Tarjetas de Indicadores (KPIs)](#tarjetas-de-indicadores-kpis)
   - [Panel de Stock por Agotarse](#panel-de-stock-por-agotarse)
   - [Panel de Movimientos Recientes](#panel-de-movimientos-recientes)
   - [Accesos Rápidos de Creación](#accesos-rápidos-de-creación)
5. [Módulo de Productos e Inventario](#5-módulo-de-productos-e-inventario)
   - [Vista Principal y Búsqueda de Productos](#vista-principal-y-búsqueda-de-productos)
   - [Estados y Clasificación de Stock](#estados-y-clasificación-de-stock)
   - [Creación de un Producto Nuevo](#creación-de-un-producto-nuevo)
   - [Presentaciones y Factores de Conversión (Múltiples Unidades)](#presentaciones-y-factores-de-conversión-múltiples-unidades)
   - [Edición y Reglas de Inmutabilidad](#edición-y-reglas-de-inmutabilidad)
   - [Archivado y Restauración de Productos](#archivado-y-restauración-de-productos)
   - [Acciones Rápidas por Producto](#acciones-rápidas-por-producto)
6. [Módulo de Movimientos de Stock](#6-módulo-de-movimientos-de-stock)
   - [Tipos de Movimiento: Entradas y Salidas](#tipos-de-movimiento-entradas-y-salidas)
   - [Movimientos Regulares vs. Ajustes de Inventario](#movimientos-regulares-vs-ajustes-de-inventario)
   - [Formulario de Registro de Movimiento](#formulario-de-registro-de-movimiento)
   - [Cálculo y Previsualización Dinámica de Stock](#cálculo-y-previsualización-dinámica-de-stock)
   - [Historial de Movimientos, Filtros Avanzados y Paginación](#historial-de-movimientos-filtros-avanzados-y-paginación)
   - [Anulación Segura y Auditable de Movimientos](#anulación-segura-y-auditable-de-movimientos)
7. [Módulo de Proveedores](#7-módulo-de-proveedores)
   - [Catálogo de Proveedores](#catálogo-de-proveedores)
   - [Alta y Edición de Proveedores](#alta-y-edición-de-proveedores)
   - [Gestión de Múltiples Líneas de Contacto](#gestión-de-múltiples-líneas-de-contacto)
   - [Archivado y Restauración de Proveedores](#archivado-y-restauración-de-proveedores)
8. [Configuración y Personalización del Sistema](#8-configuración-y-personalización-del-sistema)
   - [Límites de Stock](#límites-de-stock)
   - [Paleta de Colores de Acento](#paleta-de-colores-de-acento)
   - [Vista por Defecto al Iniciar](#vista-por-defecto-al-iniciar)
   - [Administración de Categorías (Colores e Iconos)](#administración-de-categorías-colores-e-iconos)
   - [Opciones Avanzadas: Copias de Seguridad (Backup) y Restauración](#opciones-avanzadas-copias-de-seguridad-backup-y-restauración)
9. [Reglas de Integridad y Validación de Datos](#9-reglas-de-integridad-y-validación-de-datos)
10. [Preguntas Frecuentes y Solución de Problemas (FAQ)](#10-preguntas-frecuentes-y-solución-de-problemas-faq)

---

## 1. Introducción y Arquitectura General

**Inventory Manager** es una aplicación de escritorio diseñada para brindar máxima velocidad, robustez contable y control absoluto sobre los inventarios físicos o de mercadería.

### Características Principales:
- **Rápida y Ligera**: Aplicación de escritorio diseñada para responder de forma inmediata y con mínimo consumo de recursos.
- **Almacenamiento Local 100% Autónomo**: Toda la información se guarda localmente en su equipo en el archivo de base de datos `inventory.db`. Sus datos nunca se envían a servidores externos ni a la nube, garantizando privacidad total y funcionamiento continuo sin conexión a internet.
- **Auditoría e Integridad Estricta**: No se permiten números negativos de stock, los nombres de registros se normalizan para evitar duplicidades accidentales, y las operaciones históricas quedan protegidas contra alteraciones destructivas.
- **Precisión Numérica Decimal**: Soporta hasta 6 decimales de precisión en cantidades, factores de conversión y saldos de stock (admitiendo inventarios por peso, volumen, longitud o fracciones).

---

## 2. Instalación y Primer Inicio

### Requisitos del Sistema
- **Windows**: Windows 10 o Windows 11 (64-bit).
- **Linux**: Distribuciones de 64-bit basadas en Debian/Ubuntu (`.deb`) o Fedora/RHEL/openSUSE (`.rpm`).

### Instalación en Windows y Linux
1. **Instalador de Windows (`.exe` NSIS)**: Ejecute el instalador descargado y siga las instrucciones del asistente en pantalla. Creará accesos directos en el menú Inicio y el Escritorio.
2. **Versión Portable de Windows**: No requiere instalación. Puede ejecutarse directamente desde cualquier carpeta o unidad USB.
3. **Paquetes Linux (`.deb` / `.rpm`)**:
   - En Debian/Ubuntu: `sudo dpkg -i inventory-manager_*.deb`
   - En Fedora: `sudo rpm -i inventory-manager-*.rpm`

### Ubicación de los Datos y Base de Datos Local
Toda la información del negocio se almacena en el archivo **`inventory.db`**, ubicado en el directorio estándar de datos de usuario según el sistema operativo:
- **Windows**: `%APPDATA%\Inventory Manager\inventory.db`
- **Linux**: `~/.config/inventory-manager/inventory.db`
- **macOS**: `~/Library/Application Support/Inventory Manager/inventory.db`

Una instalación nueva comienza con una base de datos limpia, sin productos ni movimientos.

### Datos de Demostración (Opcional)
Para fines de prueba o capacitación, la aplicación permite inicializar un catálogo con datos de prueba preconfigurados para explorar todas las pantallas y funciones sin alterar datos reales existentes.

---

## 3. Estructura de la Interfaz y Navegación

La ventana principal de la aplicación está dividida en tres áreas estratégicas:

```
+-----------------------------------------------------------------------+
|  [Logo] Inventory   | Header: Título de Sección      [Sol/Luna]     |
+---------------------+-------------------------------------------------+
| [📊] Dashboard       |                                                 |
| [📦] Productos       |                                                 |
| [🔄] Movimientos     |                ÁREA DE CONTENIDO                |
| [🚚] Proveedores     |                 Y VISTAS ACTIVAS                |
|                     |                                                 |
|                     |                                                 |
+---------------------+-------------------------------------------------+
```

### Barra Lateral (Sidebar)
Permite el desplazamiento inmediato entre los 4 módulos esenciales:
- **Dashboard (`/`)**: Centro de mando con métricas clave, alertas de existencias y operaciones recientes.
- **Productos (`/products`)**: Catálogo maestro de artículos, presentaciones, stock actual y estados.
- **Movimientos (`/movements`)**: Registro histórico, auditoría de entradas/salidas y cancelaciones.
- **Proveedores (`/suppliers`)**: Directorio de suministradores y datos de contacto comercial.

En la parte inferior de la barra lateral se encuentran dos accesos directos:
- **Ajustes (⚙)**: Abre la ventana modal de parámetros globales (límites, colores, categorías y respaldos).
- **Ayuda (?)**: Abre este manual de usuario.

### Cabecera (Header) y Controles Globales
Ubicada en la parte superior:
- **Indicador de Sección**: Muestra el nombre del módulo actual.
- **Selector de Tema (Sol / Luna)**: Alterna al instante entre el tema visual Claro y Oscuro.

### Modo Oscuro / Claro
La aplicación adapta toda su paleta de colores, tablas y ventanas modales según la preferencia del usuario. Su elección queda guardada en el almacenamiento local del equipo y se recordará automáticamente en cada apertura.

### Notificaciones del Sistema (Toasts) y Modales
- **Avisos Emergentes (Toasts)**: Se muestran en la esquina inferior derecha para confirmar acciones exitosas (por ejemplo, "Respaldo creado correctamente") o alertar sobre errores del sistema. Pueden cerrarse manualmente haciendo clic en el botón `×`.
- **Modales con Bloqueo de Seguridad**: Las pantallas de edición y confirmación se abren en ventanas modales focalizadas. Mientras una operación esté procesándose o guardándose, los botones se deshabilitan para evitar dobles envíos o escrituras duplicadas.

### Atajos de Teclado
| Atajo | Acción |
| --- | --- |
| `F1` | Abrir o cerrar el Manual de Usuario y Ayuda. |
| `Ctrl + O` | Abrir o cerrar la ventana de Configuración. |
| `Ctrl + T` | Alternar entre modo claro y oscuro. |
| `Ctrl + 1` | Ir al Dashboard. |
| `Ctrl + 2` | Ir a Productos. |
| `Ctrl + 3` | Ir a Movimientos. |
| `Ctrl + 4` | Ir a Proveedores. |

> Los atajos de navegación (`Ctrl + 1` a `Ctrl + 4`) se desactivan mientras hay una ventana modal abierta; el resto funciona en cualquier momento.

---

## 4. Panel de Control (Dashboard)

El **Dashboard** proporciona una visión de 360 grados del estado operativo del inventario en tiempo real.

```
+-----------------------------------------------------------------------+
| [📦 142 Productos]   [⚠️ 3 Stock bajo]   [📈 820 Movim.]   [🚚 12 Prov.]|
+------------------------------------+----------------------------------+
| Stock por agotarse       [Ver todos]| Movimientos recientes [Ver todos]|
| • Harina 0000        0.00 kg [Sin]  | [↓ Entrada] Café Molido   50 kg  |
| • Azúcar Ledesma     4.50 kg [Bajo] | [↑ Salida]  Leche Entera  12 l   |
| • Aceite de Girasol  8.00 l  [Bajo] | [↓ Entrada] Harina 000    100 kg |
+------------------------------------+----------------------------------+
| Botones Rápidos: [+ Nuevo producto]  [🔄 Registrar mov.]  [🚚 Nuevo prov.] |
+-----------------------------------------------------------------------+
```

### Tarjetas de Indicadores (KPIs)
En la parte superior se presentan cuatro tarjetas interactivas:
1. **Productos**: Cantidad total de artículos registrados y activos. Al hacer clic, navega a la vista general de Productos.
2. **Stock bajo / agotado**: Cantidad de artículos cuyas existencias están en nivel crítico o en cero. Al hacer clic, navega directamente a la lista de productos prefiltrada con los artículos urgentes.
3. **Movimientos**: Cantidad total de transacciones históricas registradas. Al hacer clic, abre el módulo de Movimientos.
4. **Proveedores**: Cantidad de empresas o proveedores comerciales dados de alta. Al hacer clic, abre la vista de Proveedores.

### Panel de Stock por Agotarse
- Lista hasta 5 productos cuyo stock actual sea menor o igual al límite "Bajo" configurado.
- Los productos se ordenan de menor a mayor existencia (los más críticos primero).
- Cada fila muestra el nombre del producto, la cantidad en su **unidad base** y la insignia de estado correspondiente (`Sin stock` o `Bajo`).
- El botón **"Ver todos"** lleva a la tabla completa filtrada por stock crítico.

### Panel de Movimientos Recientes
- Exhibe los últimos 5 movimientos registrados en la aplicación.
- Cada elemento especifica:
  - Tipo de movimiento: Insignia verde **Entrada** (con icono de flecha hacia abajo) o roja **Salida** (flecha hacia arriba).
  - Estado de anulación: Si fue anulado, se resalta explícitamente la etiqueta `Anulado`.
  - Nombre del producto.
  - Cantidad y presentación utilizada (ej. `5 Caja x 12`).
  - Fecha y hora formateada de la transacción.
- Al hacer clic sobre cualquier movimiento, la aplicación abre la vista de movimientos filtrando automáticamente por ese producto.

### Accesos Rápidos de Creación
En la base del Dashboard encontrará tres botones para iniciar las operaciones cotidianas más comunes sin necesidad de navegar por los menús:
- **Nuevo producto**: Abre el formulario de creación de artículos.
- **Registrar movimiento**: Abre la ventana de entrada/salida de stock.
- **Nuevo proveedor**: Abre la ventana de registro de suministrador.

---

## 5. Módulo de Productos e Inventario

La sección de **Productos** es el núcleo de la aplicación. Aquí se definen los artículos, sus unidades de medida, categorías y existencias.

### Vista Principal y Búsqueda de Productos
La barra de herramientas superior cuenta con herramientas de filtrado en tiempo real:
- **Casilla "Archivados"**: Al activarla, oculta los productos activos y muestra únicamente el catálogo de productos que fueron archivados.
- **Casilla "Stock bajo / agotado"**: Muestra exclusivamente los productos con existencias en niveles "Sin stock" o "Bajo".
- **Búsqueda por texto**: Permite tipear cualquier término; filtra instantáneamente por nombre del producto o por el texto contenido en sus observaciones.
- **Filtro desplegable de Categoría**: Permite aislar los productos pertenecientes a una categoría específica, ver únicamente los productos "Sin categoría" o ver "Todas las categorías".
- **Botón "Nuevo producto"**: Despliega la ventana de creación.

La tabla de datos lista los productos con su icono de categoría, nombre, observaciones, nombre de categoría, stock total en unidad base y la insignia de estado. Puede ordenar los datos por columnas haciendo clic sobre la cabecera correspondiente.

### Estados y Clasificación de Stock
El stock de cada producto se clasifica automáticamente en 4 niveles según los límites configurados:

| Estado | Condición de Stock | Color de Insignia | Significado Operativo |
| :--- | :--- | :--- | :--- |
| **Sin stock** | $\text{Stock} \le 0$ | Gris pizarra | El producto está completamente agotado. |
| **Bajo** | $0 < \text{Stock} \le \text{Límite Bajo}$ | Rojo suave / Alerta | Nivel crítico; requiere reposición inmediata. |
| **Medio** | $\text{Límite Bajo} < \text{Stock} \le \text{Límite Medio}$ | Ámbar / Precaución | Nivel normal de aviso de reposición preventiva. |
| **Alto** | $\text{Stock} > \text{Límite Medio}$ | Transparente / Verde sutil | Existencias óptimas y abundantes. |

> [!NOTE]
> Los valores por defecto de los límites son **5** para Bajo y **20** para Medio, pero pueden personalizarse a cualquier valor en la Configuración del sistema.

### Creación de un Producto Nuevo
Para crear un artículo, haga clic en el botón **"Nuevo producto"**:

1. **Nombre**: Denominación del producto (obligatorio, de 1 a 200 caracteres). El sistema normaliza automáticamente los espacios en blanco.
2. **Categoría**: Seleccione una categoría preconfigurada en el desplegable (o elija "Sin categoría").
3. **Observaciones**: Campo multilínea opcional para registrar especificaciones técnicas, código interno, ubicación en almacén, etc.
4. **Presentaciones**: Definición de cómo se fracciona o empaqueta el producto.

### Presentaciones y Factores de Conversión (Múltiples Unidades)
Un producto puede comprarse, almacenarse y retirarse en diferentes envases o formatos (por ejemplo: por unidad individual, por six-pack, por caja de 24 o por pallet de 480).

**Regla de Oro:** Todo producto debe tener **exactamente una Presentación Base cuyo factor de conversión sea 1.0**. Todas las demás presentaciones calcularán su stock multiplicando su cantidad por dicho factor.

#### Ejemplo Práctico:
Para el producto "Gaseosa Cola 500ml":
- **Presentación Base**: Nombre `Botella`, Factor `1` (Representa la unidad mínima en la que se lleva el stock).
- **Presentación 2**: Nombre `Pack x 6`, Factor `6`.
- **Presentación 3**: Nombre `Caja x 24`, Factor `24`.

Si ingresa 2 `Caja x 24`, el sistema sumará automáticamente $2 \times 24 = 48$ botellas al stock.

#### Operaciones con Presentaciones en el Formulario:
- **Añadir presentación**: Haga clic en el botón `+ Añadir`.
- **Modificar Factor**: Utilice los controles numéricos o escriba el valor deseado. Se admiten hasta 6 decimales (por ejemplo, factores como `0.5` o `1.250`).
- **Eliminar presentación**: Haga clic en el icono de papelera junto a la fila correspondiente (el producto no permite eliminar la última presentación existente).

### Edición y Reglas de Inmutabilidad
Para editar un producto existente, haga clic directamente sobre su fila en la tabla o pulse el botón del lápiz (`Editar`).

> [!IMPORTANT]
> **Protección Histórica de Factores y Unidad Base**:
> Si un producto ya posee movimientos de stock registrados:
> 1. **No se puede cambiar la presentación base**: La unidad base queda fija para no desvirtuar las operaciones anteriores.
> 2. **No se puede modificar el factor de una presentación ya utilizada**: Si una presentación ya intervino en movimientos previos, su factor no puede alterarse. Si necesita un factor nuevo (por ejemplo, una caja que ahora viene de 20 unidades en vez de 24), cree una **nueva presentación** (ej. "Caja x 20") y retire la anterior.

### Archivado y Restauración de Productos
En Inventory Manager los registros no se destruyen físicamente, para evitar que los reportes y movimientos pasados queden huérfanos. En su lugar, se utiliza el **Archivado (Borrado Lógico)**.

- **Requisito para Archivar**: Un producto **solo se puede archivar si su stock actual es exactamente 0.00**. Si aún posee mercadería en existencia, el botón de papelera permanecerá deshabilitado. Deberá registrar un movimiento de Salida o Ajuste antes de poder archivarlo.
- **Cómo archivar**: Haga clic en el icono de papelera (`Archivar`). El sistema solicitará confirmación mediante un diálogo de seguridad.
- **Cómo restaurar**:
  1. Marque la casilla **"Archivados"** en la barra superior.
  2. Localice el producto en la lista.
  3. Presione el botón **"Restaurar"**. El producto volverá inmediatamente al catálogo activo con todo su historial intacto.

### Acciones Rápidas por Producto
En el extremo derecho de cada fila de la tabla de productos encontrará botones de acción directa:
- **`+` (Registrar entrada)**: Abre el formulario de movimiento con este producto preseleccionado y el tipo Entrada por defecto.
- **`−` (Registrar salida)**: Abre el mismo formulario con este producto preseleccionado y el tipo Salida por defecto.
- **`Historial` (Icono de reloj con flecha)**: Navega directamente a la tabla de Movimientos, aplicando el filtro exclusivo para ver todo el historial de este producto.
- **`Lápiz` (Editar)**: Abre el editor de producto y presentaciones.
- **`Papelera` (Archivar)**: Archiva el producto (habilitado solo si el stock es cero).

---

## 6. Módulo de Movimientos de Stock

El módulo de **Movimientos** garantiza la trazabilidad total de cada incremento o deducción en el inventario.

### Tipos de Movimiento: Entradas y Salidas
- **Entrada (`in`)**: Incrementa las existencias del producto en la base de datos. Admite vincular un Proveedor como origen de la mercadería.
- **Salida (`out`)**: Descuenta existencias del producto. **No admite proveedor**, ya que representa consumos, ventas, mermas o despachos.

### Movimientos Regulares vs. Ajustes de Inventario
Al registrar un movimiento, existe una casilla de verificación denominada **"Ajuste de inventario"**:
- **Movimiento Regular**: Corresponde a la operatoria habitual del negocio (compras de reposición, ventas o consumos normales).
- **Ajuste de Inventario**: Utilizado para conciliaciones físicas de almacén (inventario rotativo, roturas, faltantes o sobrantes de auditoría). Al marcar esta opción, el campo **"Motivo del ajuste" se vuelve estrictamente obligatorio** para garantizar que toda discrepancia quede documentada y justificada ante auditorías.

### Formulario de Registro de Movimiento
Para asentar una operación, pulse **"Nuevo movimiento"** en la barra de herramientas o en el Dashboard:

1. **Producto**: Seleccione el producto de la lista desplegable.
2. **Presentación**: Seleccione el empaque utilizado. Cada opción muestra su factor de conversión (ej. `Bolsa x 5 (×5)`).
3. **Cantidad**: Número de unidades o paquetes que ingresan o egresan.
4. **Tipo**: Seleccione `Entrada` o `Salida`.
5. **Casilla "Ajuste de inventario"**: Marque si corresponde a una rectificación de inventario y complete el motivo.
6. **Proveedor**: Disponible únicamente en Entradas. Seleccione el proveedor responsable o deje en "Sin proveedor".
7. **Notas**: Observaciones complementarias opcionales (número de remito, factura de compra, destino interno, etc.).

### Cálculo y Previsualización Dinámica de Stock
Dentro de la misma ventana de registro, el sistema incluye un cuadro de **previsualización en tiempo real**:

```
+-------------------------------------------------------------+
| Actual:     10.00 kg                                        |
| Operación:  2.00 × 5.00 = 10.00 kg                          |
| Resultante: 20.00 kg                                        |
+-------------------------------------------------------------+
```

- Muestra las existencias actuales en unidad base.
- Multiplica en vivo la cantidad por el factor de la presentación elegida.
- Proyecta el stock resultante final.
- **Protección contra stock negativo**: En las salidas, si la cantidad solicitada supera las existencias disponibles, el valor resultante se resalta en rojo de advertencia y el botón "Registrar" queda inhabilitado. El campo de cantidad limita automáticamente el tope máximo retirable según las unidades enteras disponibles.

### Historial de Movimientos, Filtros Avanzados y Paginación
La vista de movimientos ofrece un potente panel de filtros combinables para auditoría forense y control diario:

```
+-------------------------------------------------------------------------------------------------------------+
| [Todos] [Entradas] [Salidas]                                                        [+ Nuevo movimiento]    |
+-------------------------------------------------------------------------------------------------------------+
| Buscar: [                ]  Producto: [Todos ▼]  Proveedor: [Todos ▼]                                   |
| Desde:  [DD/MM/AAAA      ]  Hasta:    [DD/MM/AAAA    ]  Estado:      [Todos ▼]                              |
+-------------------------------------------------------------------------------------------------------------+
```

- **Selector Rápido de Tipo**: Botones para ver `Todos`, únicamente `Entradas` o únicamente `Salidas`.
- **Buscador de Texto**: Coincide en tiempo real con nombre de producto, proveedor o notas.
- **Filtro por Producto**: Aísla todas las operaciones del producto elegido.
- **Filtro por Proveedor**: Muestra las compras realizadas a un proveedor específico.
- **Rango de Fechas (Desde / Hasta)**: Acota las transacciones al período cronológico seleccionado.
- **Estado**: Permite filtrar entre transacciones `Vigentes` o `Anuladas`.
- **Paginación Inteligente**: Las consultas se resuelven en la base de datos a razón de **25 filas por página**, garantizando un rendimiento instantáneo incluso con cientos de miles de movimientos. Utilice los botones `Anterior` y `Siguiente` en el pie de página.

### Anulación Segura y Auditable de Movimientos
Los movimientos de stock **nunca se eliminan** de la base de datos. Si se cometió un error de tipeo o se devolvió una mercadería, el movimiento debe ser **Anulado**.

#### Procedimiento de Anulación:
1. Localice el movimiento en la tabla y presione el icono rojo de papelera (`Anular movimiento`).
2. Se desplegará la ventana modal **"Anular movimiento"**.
3. Ingrese obligatoriamente el **Motivo de la anulación** (ej. "Error en la cantidad cargada en remito", "Mercadería rechazada por embalaje dañado").
4. Confirme la operación.

#### Efectos de la Anulación:
- El movimiento se marca con fecha y hora de anulación (`cancelled_at`) y conserva su motivo de anulación.
- Su estado visual cambia a `Anulado`.
- **Reversión atómica de stock**: Si el movimiento anulado era una **Entrada**, el sistema descuenta automáticamente la cantidad del stock; si era una **Salida**, la restituye al stock.
- **Seguridad**: La anulación será rechazada por el sistema si revertir una Entrada dejara el stock en números negativos (por ejemplo, si ingresaron 10 unidades pero ya se consumieron 8, no se puede anular la entrada original de 10 porque faltaría mercadería).

---

## 7. Módulo de Proveedores

El módulo de **Proveedores** permite gestionar el directorio de empresas, fabricantes o distribuidores que abastecen su negocio.

### Catálogo de Proveedores
Presenta una tabla clara con:
- **Nombre de la empresa o proveedor**.
- **Líneas de contacto** (separadas limpiamente por puntos de viñeta `·`).
- **Notas u observaciones comerciales**.

### Alta y Edición de Proveedores
Para dar de alta un nuevo proveedor, pulse **"Nuevo proveedor"**:
1. **Nombre**: Denominación social o comercial (obligatorio, único en el sistema).
2. **Contactos**: Permite agregar múltiples canales de comunicación dinámicos.
3. **Notas**: Condiciones de pago, horarios de entrega, dirección del depósito, etc.

Para modificar un proveedor existente, pulse el icono del lápiz en su fila.

### Gestión de Múltiples Líneas de Contacto
A diferencia de sistemas rígidos que solo permiten un teléfono, Inventory Manager permite agregar todas las líneas necesarias para cada proveedor:
- Haga clic en el botón `+ Añadir` dentro de la sección "Contactos".
- Cargue teléfonos, correos electrónicos, cuentas de WhatsApp, nombres de vendedores o direcciones.
- Para eliminar una línea sobrante, pulse el icono de papelera al lado del campo correspondiente.

### Archivado y Restauración de Proveedores
- Si un proveedor deja de operar con su empresa, puede archivarlo pulsando el icono de papelera (`Archivar`).
- El historial de compras y movimientos asociados a dicho proveedor **se preservará intacto**.
- Para consultar o recuperar proveedores dados de baja, marque la casilla **"Archivados"** en la parte superior y presione **"Restaurar"**.

---

## 8. Configuración y Personalización del Sistema

Haga clic en el botón **Ajustes (⚙)** ubicado en la parte inferior de la barra lateral, junto al botón **Ayuda**, para abrir la ventana modal de **Configuración**.

```
+-------------------------------------------------------------------+
| CONFIGURACIÓN                                                 [X] |
+-------------------------------------------------------------------+
| LÍMITES DE STOCK                                                  |
| Límite bajo: [ 5.00 ]         Límite medio: [ 20.00 ]             |
|                                            [Guardar límites]      |
|                                                                   |
| COLOR DE ACENTO                                                   |
| (• Azul) (Violeta) (Esmeralda) (Ámbar) (Rosa) (Teal) (Gris)       |
|                                                                   |
| VISTA POR DEFECTO                                                 |
| [ Dashboard                 ▼ ]                                   |
|                                                                   |
| CATEGORÍAS                                                        |
| [+ Nueva categoría]                                               |
| • [📦 Alimentos]       [✏️] [🗑️]                                    |
| • [🥤 Bebidas]         [✏️] [🗑️]                                    |
|                                                                   |
| AVANZADO (Base de Datos)                                          |
| [💾 Crear respaldo]   [📂 Restaurar respaldo]                     |
+-------------------------------------------------------------------+
```

### Límites de Stock
Permite parametrizar las reglas que determinan las insignias de estado de todos los productos:
- **Límite bajo**: Nivel a partir del cual el stock se considera crítico (ej. 5 unidades).
- **Límite medio**: Nivel a partir del cual el stock se considera en aviso preventivo (ej. 20 unidades).
- **Validación matemática**: Debe cumplirse siempre la regla:
  $$0 \le \text{Límite Bajo} < \text{Límite Medio} \le 1.000.000.000$$
- Pulse **"Guardar límites"** para aplicar los cambios en todo el inventario.

### Paleta de Colores de Acento
Personalice la identidad visual de la aplicación según sus preferencias estéticas o institucionales. Se ofrecen 7 temas cromáticos diseñados para máxima legibilidad tanto en modo claro como en modo oscuro:
1. **Azul** (predeterminado)
2. **Violeta**
3. **Esmeralda**
4. **Ámbar**
5. **Rosa**
6. **Teal**
7. **Gris**

Al hacer clic sobre cualquier círculo de color, los botones principales, enlaces, bordes activos y elementos destacados cambiarán inmediatamente.

### Vista por Defecto al Iniciar
Define qué pantalla se abrirá automáticamente cada vez que inicie la aplicación:
- **Dashboard** (Recomendado para supervisores y administradores).
- **Productos** (Ideal para operarios de almacén o repositores).
- **Movimientos** (Para puestos de caja, carga de remitos o control de despachos).
- **Proveedores**.

### Administración de Categorías (Colores e Iconos)
Las categorías permiten organizar el catálogo y dotar a cada producto de un icono y color representativo en las tablas.

#### Creación de Categorías:
1. Pulse **"Nueva categoría"**.
2. Escriba el **Nombre** (ej. "Lácteos", "Herramientas", "Limpieza").
3. Seleccione uno de los 8 **Colores de categoría**.
4. Utilice el **Selector de Iconos (IconPicker)**: Puede buscar entre decenas de iconos disponibles (cajas, botellas, alimentos, herramientas, vehículos, compras, etc.).
5. Pulse **"Crear"**.

#### Eliminación de Categorías:
Al eliminar una categoría con el icono de papelera, **los productos asociados no se borran**; simplemente pasan al estado "Sin categoría" para salvaguardar sus existencias y datos.

### Opciones Avanzadas: Copias de Seguridad (Backup) y Restauración
Proteja la información de su negocio contra pérdidas fortuitas o traslade su inventario a otra computadora con facilidad:

#### 1. Crear Respaldo de la Base de Datos (`Crear respaldo`):
- Abre una ventana del sistema para elegir la carpeta y el nombre del archivo.
- Sugiere un nombre con marca de tiempo automática (ej. `inventario-20260913-183000.db`).
- Genera una **copia exacta, consistente y desfragmentada** del archivo de datos incluso mientras el sistema está en uso.

#### 2. Restaurar Respaldo (`Restaurar respaldo`):
- Seleccione un archivo `.db` generado previamente.
- **Verificación de Seguridad Previa**: La aplicación verifica que el archivo sea un respaldo válido y compatible con la estructura del sistema.
- **Copia de Seguridad Preventiva Automática**: Antes de reemplazar los datos, la aplicación genera automáticamente una copia de resguardo de emergencia (`pre-restore-backup.db`) en el directorio de la aplicación. Si el archivo estuviese corrupto, el sistema recupera automáticamente sus datos previos para evitar cualquier pérdida.

---

## 9. Reglas de Integridad y Validación de Datos

Inventory Manager implementa estrictas reglas contables y de software para garantizar la coherencia absoluta de la información:

1. **Unicidad y Normalización Unicode (NFC)**:
   - Los nombres de productos, proveedores y categorías se normalizan a minúsculas y se colapsan los espacios repetidos para verificar duplicados.
   - *Importante*: Los acentos (tildes) y la letra `ñ` se consideran caracteres distintos (ej. "Café" y "Cafe" son nombres diferentes).
   - **Nombres en Registros Archivados**: Un nombre reservado por un registro archivado no puede ser reutilizado por un registro nuevo. La aplicación le advertirá con el mensaje:  
     `"Ya existe un producto/proveedor con ese nombre (puede estar archivado)"`  
     y le ofrecerá el botón directo **"Ver registro existente (archivado)"** para restaurarlo en 1 solo clic.
2. **Presentaciones Únicas por Producto**:
   - Dentro de un mismo producto no pueden existir dos presentaciones con el mismo nombre (ej. no puede haber dos presentaciones llamadas "Caja").
3. **No Negatividad de Stock**:
   - El stock jamás puede ser inferior a 0. Las salidas que superen las existencias son rechazadas automáticamente por las restricciones a nivel de base de datos.
4. **Límites Numéricos y Precisión**:
   - Cantidades, factores y saldos de stock admiten hasta **6 lugares decimales** (mínimo $0.000001$) y un valor máximo de **1.000.000.000** (mil millones).
   - Las operaciones matemáticas redondean a 6 decimales para evitar residuos de coma flotante binaria.
5. **Transacciones Atómicas (ACID)**:
   - Cada alta de producto con sus presentaciones o cada movimiento con su actualización de stock se procesa dentro de una transacción segura y atómica. Si algún paso falla, se revierte todo al estado anterior sin dejar datos corruptos.

---

## 10. Preguntas Frecuentes y Solución de Problemas (FAQ)

### ¿Por qué la aplicación me dice que un producto "ya existe" si no lo veo en la lista?
Lo más probable es que el producto haya sido archivado en el pasado. Para comprobarlo:
1. En la pantalla de Productos, marque la casilla **"Archivados"** en la barra superior.
2. Busque el nombre del producto.
3. Si lo encuentra, haga clic en **"Restaurar"** para volver a utilizarlo con su historial previo.
*Tip: El propio formulario de creación le mostrará un botón que dice `Ver registro existente (archivado)` si detecta la coincidencia.*

### ¿Por qué no puedo cambiar el factor de conversión o la unidad base de un producto?
Porque el producto ya posee movimientos de stock registrados en el historial. Modificar un factor retroactivamente alteraría el saldo histórico de las compras y consumos pasados, violando la integridad de auditoría. Si el fabricante cambió el formato de empaque (por ejemplo, ahora la caja trae 10 unidades en lugar de 12), cree una nueva presentación llamada "Caja x 10" con factor 10 y comience a registrar los movimientos con esa nueva presentación.

### ¿Por qué no puedo archivar un producto?
Para poder archivar un producto, su **stock debe ser estrictamente cero (0)**. Esto previene que mercadería física quede olvidada u oculta en el sistema. Registre un movimiento de Salida o Ajuste de inventario para llevar el stock a cero; una vez en cero, el botón de papelera se habilitará automáticamente.

### ¿Puedo eliminar un movimiento si me equivoqué en la cantidad?
No se pueden borrar movimientos para preservar la trazabilidad histórica. Lo que debe hacer es **Anular el movimiento** (icono de papelera en la tabla de Movimientos) indicando el motivo. El sistema revertirá el stock automáticamente. Luego podrá registrar el movimiento correcto.

### ¿Qué ocurre si intento anular una Entrada de stock pero ya no tengo suficiente mercadería?
El sistema impedirá la anulación con el mensaje:  
`"No se puede anular: stock insuficiente o producto archivado."`  
Esto ocurre porque anular una Entrada implica restar ese stock, y si ya consumió esas unidades mediante Salidas posteriores, el stock caería por debajo de cero. Deberá regularizar primero las existencias físicas mediante un Ajuste.

### ¿Cómo paso mi inventario a otra computadora?
1. En la computadora de origen, abra **Configuración (⚙)** > sección **Avanzado** y haga clic en **"Crear respaldo"**.
2. Guarde el archivo `.db` resultante en un pendrive o envíelo a su nuevo equipo.
3. Instale Inventory Manager en la nueva computadora.
4. Abra **Configuración (⚙)** > **"Restaurar respaldo"** y elija el archivo `.db`.
5. La aplicación recargará todos los datos instantáneamente.

---

*Manual elaborado para Inventory Manager v0.9.0 o superior.*
