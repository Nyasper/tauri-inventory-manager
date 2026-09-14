<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Badge from "$lib/components/ui/Badge.svelte";
  import StockBadge from "$lib/components/ui/StockBadge.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import type { Shortcut } from "$lib/services/keybindings.service";
  import {
    BookOpen,
    LayoutDashboard,
    Boxes,
    ArrowLeftRight,
    Truck,
    Settings,
    CircleHelp,
    Search,
    ShieldCheck,
    Layers,
    TriangleAlert,
    Info,
    Check,
    DatabaseBackup,
    Trash,
  } from "@lucide/svelte";

  type SectionId =
    | "general"
    | "dashboard"
    | "products"
    | "movements"
    | "suppliers"
    | "settings"
    | "faq";

  interface SectionMeta {
    id: SectionId;
    title: string;
    icon: typeof BookOpen;
    badge?: string;
  }

  const SECTIONS: SectionMeta[] = [
    { id: "general", title: "Visión General", icon: BookOpen },
    { id: "dashboard", title: "Dashboard", icon: LayoutDashboard },
    { id: "products", title: "Productos y Stock", icon: Boxes },
    { id: "movements", title: "Movimientos", icon: ArrowLeftRight },
    { id: "suppliers", title: "Proveedores", icon: Truck },
    { id: "settings", title: "Configuración", icon: Settings },
    { id: "faq", title: "Preguntas Frecuentes", icon: CircleHelp, badge: "FAQ" },
  ];

  let { shortcuts = [] }: { shortcuts?: Shortcut[] } = $props();

  let activeSection = $state<SectionId>("general");
  let searchQuery = $state("");

  // Search filter helper
  const filteredSections = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    if (!q) return SECTIONS;
    return SECTIONS.filter(s => {
      if (s.title.toLowerCase().includes(q)) return true;
      if (s.id.toLowerCase().includes(q)) return true;
      return false;
    });
  });

  function selectSection(id: SectionId) {
    activeSection = id;
  }
</script>

<Modal title="Manual de Usuario y Ayuda" onClose={() => modalService.close()} width="xl">
  <div class="help-container">
    <!-- Barra de navegación lateral del manual -->
    <nav class="help-nav" aria-label="Secciones del manual">
      <div class="search-box">
        <Search size={15} />
        <input
          type="text"
          class="search-input"
          placeholder="Buscar tema..."
          bind:value={searchQuery}
          aria-label="Buscar en el manual"
        />
      </div>

      <div class="nav-list">
        {#each filteredSections as section (section.id)}
          {@const Icon = section.icon}
          <button
            type="button"
            class="nav-tab"
            class:active={activeSection === section.id}
            onclick={() => selectSection(section.id)}
          >
            <Icon size={16} />
            <span class="tab-label">{section.title}</span>
            {#if section.badge}
              <span class="tab-badge">{section.badge}</span>
            {/if}
          </button>
        {:else}
          <p class="nav-empty">Sin resultados para «{searchQuery.trim()}».</p>
        {/each}
      </div>

      <div class="nav-footer">
        <span class="version-label">Gestor de Inventario</span>
        <span class="sub-label">Control de Inventario Local</span>
      </div>
    </nav>

    <!-- Área de contenido de la sección activa -->
    <article class="help-content">
      {#if activeSection === "general"}
        <div class="topic">
          <h2><BookOpen size={20} class="topic-icon" /> Introducción</h2>
          <p class="lead">
            <strong>Gestor de Inventario</strong> es una aplicación de escritorio diseñada para brindar control
            total, ágil y seguro sobre las existencias de productos, presentaciones fraccionadas,
            trazabilidad histórica de movimientos y gestión de proveedores.
          </p>

          <div class="card-grid">
            <div class="info-card">
              <div class="card-header">
                <ShieldCheck size={18} class="accent-icon" />
                <h3>Almacenamiento 100% Local</h3>
              </div>
              <p>
                Toda la información se almacena localmente en su equipo en el archivo <code>inventory.db</code>.
                No depende de servidores externos ni sube información a la nube, garantizando máxima
                privacidad y funcionamiento ininterrumpido sin conexión a internet.
              </p>
            </div>

            <div class="info-card">
              <div class="card-header">
                <Layers size={18} class="accent-icon" />
                <h3>Precisión de 6 Decimales</h3>
              </div>
              <p>
                Cantidades, factores de conversión y saldos admiten hasta 6 lugares decimales
                (por ejemplo, kilos, litros o fracciones métricas), con un límite de hasta 1.000 millones.
              </p>
            </div>
          </div>

          <h3>Estructura de la Interfaz</h3>
          <ul class="feature-list">
            <li>
              <strong>Barra Lateral (Sidebar):</strong> Permite saltar al instante entre el Dashboard,
              Productos, Movimientos y Proveedores. En su pie se encuentran los accesos a
              <em>Ajustes</em> y <em>Ayuda</em>.
            </li>
            <li>
              <strong>Cabecera (Header):</strong> Incluye el conmutador de <em>Modo Claro / Modo Oscuro</em>.
            </li>
            <li>
              <strong>Botón de Ayuda (?):</strong> Ubicado abajo a la izquierda del menú para consultar este
              manual en cualquier momento (también accesible mediante la tecla <strong>F1</strong>).
            </li>
          </ul>

          <h3>Atajos de Teclado</h3>
          <ul class="feature-list">
            {#each shortcuts as shortcut (shortcut.id)}
              <li>
                <strong>{shortcut.keys}:</strong> {shortcut.description}.
              </li>
            {/each}
          </ul>
          <p class="shortcuts-note">
            Los atajos de navegación (Ctrl + 1 a Ctrl + 4) se desactivan mientras hay una ventana modal
            abierta; los demás funcionan en cualquier momento.
          </p>

          <div class="tip-box">
            <Info size={18} />
            <div>
              <strong>¿Dónde están mis datos?</strong> Se almacenan en la carpeta local de datos de la aplicación
              (<code>inventory.db</code>). Puedes crear copias de seguridad en cualquier momento desde
              <em>Configuración &gt; Avanzado</em>.
            </div>
          </div>
        </div>

      {:else if activeSection === "dashboard"}
        <div class="topic">
          <h2><LayoutDashboard size={20} class="topic-icon" /> Panel de Control (Dashboard)</h2>
          <p class="lead">
            El Dashboard es el centro de supervisión operativa. Ofrece un panorama instantáneo
            de los indicadores más importantes y accesos directos a las tareas habituales.
          </p>

          <h3>Tarjetas de Indicadores (KPIs)</h3>
          <div class="card-grid">
            <div class="kpi-demo tone-primary">
              <span class="kpi-title">Productos</span>
              <span class="kpi-desc">Total de productos activos en el catálogo. Clic para ir al listado.</span>
            </div>
            <div class="kpi-demo tone-danger">
              <span class="kpi-title">Stock bajo / agotado</span>
              <span class="kpi-desc">Artículos en nivel crítico o sin stock. Clic para prefiltrar la lista.</span>
            </div>
            <div class="kpi-demo tone-primary">
              <span class="kpi-title">Movimientos</span>
              <span class="kpi-desc">Histórico de transacciones registradas en el sistema.</span>
            </div>
            <div class="kpi-demo tone-primary">
              <span class="kpi-title">Proveedores</span>
              <span class="kpi-desc">Empresas o suministradores dados de alta.</span>
            </div>
          </div>

          <h3>Paneles Informativos</h3>
          <ul class="feature-list">
            <li>
              <strong>Stock por agotarse:</strong> Muestra los 10 productos con menor existencia relativa a
              sus límites. Cuenta con el enlace <em>"Ver todos"</em> para aislar los productos que
              precisan reposición urgente.
            </li>
            <li>
              <strong>Movimientos recientes:</strong> Lista las últimas 10 operaciones asentadas, detallando
              tipo (Entrada verde / Salida roja), cantidad en su presentación y fecha. Si el
              movimiento fue anulado, se resalta de forma transparente.
            </li>
          </ul>

          <h3>Accesos Rápidos Inferiores</h3>
          <p>
            Al pie del Dashboard dispones de tres botones de acción inmediata: <em>Nuevo producto</em>,
            <em>Registrar movimiento</em> y <em>Nuevo proveedor</em>.
          </p>
        </div>

      {:else if activeSection === "products"}
        <div class="topic">
          <h2><Boxes size={20} class="topic-icon" /> Módulo de Productos e Inventario</h2>
          <p class="lead">
            Aquí administras el catálogo maestro, las presentaciones comerciales, los niveles de stock
            y el estado de existencias.
          </p>

          <h3>Clasificación de Estados de Stock</h3>
          <table class="help-table">
            <thead>
              <tr>
                <th>Estado</th>
                <th>Insignia</th>
                <th>Criterio de Existencias</th>
                <th>Significado</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td><strong>Sin stock</strong></td>
                <td><StockBadge status="out" /></td>
                <td>Stock &le; 0</td>
                <td>Producto agotado completamente.</td>
              </tr>
              <tr>
                <td><strong>Bajo</strong></td>
                <td><StockBadge status="low" /></td>
                <td>0 &lt; Stock &le; Límite bajo</td>
                <td>Nivel crítico; requiere reposición pronta.</td>
              </tr>
              <tr>
                <td><strong>Medio</strong></td>
                <td><StockBadge status="medium" /></td>
                <td>Límite bajo &lt; Stock &le; Límite medio</td>
                <td>Aviso preventivo de reposición regular.</td>
              </tr>
              <tr>
                <td><strong>Alto</strong></td>
                <td><StockBadge status="high" /></td>
                <td>Stock &gt; Límite medio</td>
                <td>Existencias óptimas y suficientes.</td>
              </tr>
            </tbody>
          </table>

          <h3>Presentaciones y Factores de Conversión</h3>
          <p>
            Permite contabilizar un mismo artículo en diferentes formatos de compra o venta (unidades,
            packs, cajas, pallets, kilos, litros).
          </p>
          <div class="warning-box">
            <TriangleAlert size={18} />
            <div>
              <strong>Regla fundamental:</strong> Todo producto debe tener <strong>exactamente una presentación base con factor 1</strong>.
              Todas las demás presentaciones calcularán su stock multiplicando su cantidad por el factor correspondiente
              (ej. una <em>Caja x 12</em> tiene factor <code>12</code>; ingresar 2 cajas suma 24 unidades base).
            </div>
          </div>

          <h3>Reglas de Integridad y Protección Histórica</h3>
          <ul class="feature-list">
            <li>
              <strong>Inmutabilidad de factores usados:</strong> Si un producto ya tiene movimientos
              registrados, no es posible cambiar su unidad base ni alterar el factor de una presentación
              ya utilizada. Para un nuevo formato de empaque, crea una nueva presentación.
            </li>
            <li>
              <strong>Requisito estricto para archivar:</strong> Un producto <strong>solo puede ser archivado si su stock actual es 0</strong>.
              Si aún quedan existencias, realiza una Salida o Ajuste antes de archivar.
            </li>
            <li>
              <strong>Eliminación definitiva:</strong> Con la casilla <em>"Archivados"</em> activa puedes
              restaurar el producto o eliminarlo para siempre. Sus movimientos se conservan en el historial
              marcados como <em>"Producto eliminado"</em>.
            </li>
            <li>
              <strong>Recuperación de duplicados:</strong> Si intentas registrar un producto con un nombre
              que ya existe (incluso si está archivado), el sistema te ofrecerá el botón directo
              <em>"Ver registro existente (archivado)"</em> para restaurarlo con un solo clic.
            </li>
          </ul>

          <h3>Acciones Rápidas en la Tabla</h3>
          <p>
            Al pasar el cursor sobre un producto dispones de accesos para:
            <code>+</code> registrar una entrada directa, <code>−</code> registrar una salida
            con el producto preseleccionado, <code>Movimientos</code> para ver su historial,
            <code>Lápiz</code> para editar y <code>Papelera</code> para archivar (si el stock es cero).
            En la vista <em>"Archivados"</em>, cada fila ofrece <em>"Restaurar"</em> y el icono de
            papelera para eliminarlo definitivamente.
          </p>
        </div>

      {:else if activeSection === "movements"}
        <div class="topic">
          <h2><ArrowLeftRight size={20} class="topic-icon" /> Movimientos de Stock y Auditoría</h2>
          <p class="lead">
            Registra cada entrada o salida de inventario, garantizando trazabilidad total e imposibilidad
            de saldos negativos.
          </p>

          <h3>Tipos de Operación</h3>
          <div class="card-grid">
            <div class="info-card">
              <div class="card-header">
                <Badge bg="var(--success)" color="#ffffff">Entrada</Badge>
                <h3>Entrada de Stock</h3>
              </div>
              <p>
                Suma existencias al producto. Permite seleccionar un <strong>Proveedor</strong> para asociar
                el origen de la compra o recepción.
              </p>
            </div>

            <div class="info-card">
              <div class="card-header">
                <Badge bg="var(--danger)" color="#ffffff">Salida</Badge>
                <h3>Salida de Stock</h3>
              </div>
              <p>
                Descuenta existencias del producto. No admite proveedor. Limita el ingreso de cantidades
                para impedir que el stock quede en números negativos.
              </p>
            </div>
          </div>

          <h3>Ajustes de Inventario</h3>
          <p>
            Al marcar la casilla <em>"Ajuste de inventario"</em> en el formulario, el campo
            <strong>"Motivo del ajuste" se vuelve obligatorio</strong>. Es la forma correcta y auditable
            de asentar roturas, diferencias de conteo físico, mermas o sobrantes de auditoría.
          </p>

          <h3>Previsualización Dinámica</h3>
          <p>
            El formulario muestra en tiempo real:
            <code>Stock actual</code> &rarr; <code>Operación (cantidad × factor)</code> &rarr; <code>Stock resultante</code>.
            Si el resultado fuera negativo, la interfaz advierte en color rojo y desactiva el botón de registro.
          </p>

          <h3>Anulación Segura de Movimientos</h3>
          <div class="tip-box">
            <Info size={18} />
            <div>
              Si se cometió un error, pulsa el icono de papelera en la tabla para <strong>Anular</strong>.
              Deberás ingresar un motivo de anulación obligatorio. El sistema revertirá el stock de forma
              atómica y mantendrá el registro original para fines de auditoría.
              <br />
              Como excepción de mantenimiento, un administrador puede purgar el historial en lote desde
              <em>Configuración &gt; Avanzado &gt; "Eliminar movimientos"</em> con contraseña.
            </div>
          </div>

          <h3>Filtros y Paginación</h3>
          <p>
            La vista de movimientos cuenta con filtros por tipo (Todos / Entradas / Salidas), buscador de texto,
            selectores de producto y proveedor, filtro por rango de fechas (Desde / Hasta), y
            filtro de estado (Vigentes / Anulados). Las consultas se resuelven de forma paginada a 25 registros
            por página.
          </p>
          <p>
            Los movimientos de productos eliminados definitivamente conservan su nombre y muestran la
            etiqueta <em>"Producto eliminado"</em>; siguen apareciendo en la búsqueda por texto aunque ya no
            estén disponibles en el filtro por producto.
          </p>
        </div>

      {:else if activeSection === "suppliers"}
        <div class="topic">
          <h2><Truck size={20} class="topic-icon" /> Módulo de Proveedores</h2>
          <p class="lead">
            Centraliza los datos de fabricantes, distribuidores y contactos comerciales asociados
            a las compras de mercadería.
          </p>

          <h3>Características Principales</h3>
          <ul class="feature-list">
            <li>
              <strong>Nombre único:</strong> Identifica a la empresa o distribuidor.
            </li>
            <li>
              <strong>Múltiples canales de contacto:</strong> Puedes añadir tantas líneas de contacto como
              necesites (teléfonos fijos, celulares, WhatsApp, emails o direcciones) pulsando <em>"+ Añadir"</em>.
            </li>
            <li>
              <strong>Observaciones comerciales:</strong> Registra condiciones de pago, plazos de entrega,
              horarios o nombres de vendedores.
            </li>
            <li>
              <strong>Archivado sin pérdida histórica:</strong> Al archivar un proveedor, los movimientos y
              entradas pasadas siguen conservando su nombre. Puedes consultar los proveedores archivados y
              restaurarlos activando la casilla <em>"Archivados"</em> en la barra superior.
            </li>
            <li>
              <strong>Registro rápido de entradas:</strong> El botón <code>+</code> de la fila abre el formulario
              de movimiento con el proveedor ya seleccionado y el tipo Entrada, asociando la compra al instante.
            </li>
          </ul>
        </div>

      {:else if activeSection === "settings"}
        <div class="topic">
          <h2><Settings size={20} class="topic-icon" /> Configuración y Opciones Avanzadas</h2>
          <p class="lead">
            Ajusta los parámetros globales de la aplicación, límites de stock, estética, respaldos
            de la base de datos y eliminación masiva de movimientos.
          </p>

          <div class="card-grid">
            <div class="info-card">
              <h3>Límites de Stock</h3>
              <p>
                Configura los límites numéricos de stock <strong>Bajo</strong> y <strong>Medio</strong>.
                Debe cumplirse la relación: <code>0 &le; Bajo &lt; Medio</code>.
              </p>
            </div>

            <div class="info-card">
              <h3>Color de Acento</h3>
              <p>
                Elige entre 7 temas visuales diseñados profesionalmente: <em>Azul, Violeta, Esmeralda,
                Ámbar, Rosa, Teal o Gris</em>. La preferencia se guarda de inmediato.
              </p>
            </div>

            <div class="info-card">
              <h3>Vista por Defecto</h3>
              <p>
                Define qué pantalla se abre al iniciar la aplicación (Dashboard, Productos, Movimientos
                o Proveedores).
              </p>
            </div>

            <div class="info-card">
              <h3>Categorías</h3>
              <p>
                Personaliza el catálogo de categorías con colores distintivos y un catálogo de más de
                60 iconos. Al eliminar una categoría, sus productos pasan a "Sin categoría" sin perderse.
              </p>
            </div>

            <div class="info-card">
              <div class="card-header">
                <DatabaseBackup size={18} class="accent-icon" />
                <h3>Copias de Seguridad</h3>
              </div>
              <p>
                <strong>Crear respaldo:</strong> Exporta un archivo <code>.db</code> consistente.<br />
                <strong>Restaurar respaldo:</strong> Restaura una copia previa. El sistema genera una
                copia de seguridad preventiva automática antes de sobreescribir.
              </p>
            </div>

            <div class="info-card">
              <div class="card-header">
                <Trash size={18} class="accent-icon" />
                <h3>Eliminar Movimientos</h3>
              </div>
              <p>
                Purga los <strong>N movimientos más recientes o más antiguos</strong> del historial. Requiere la
                contraseña de eliminación (inicial <code>admin</code>), que puedes cambiar en la misma ventana
                con la clave actual y la nueva dos veces. El stock actual no cambia.
              </p>
            </div>
          </div>
        </div>

      {:else if activeSection === "faq"}
        <div class="topic">
          <h2><CircleHelp size={20} class="topic-icon" /> Preguntas Frecuentes (FAQ)</h2>
          <p class="lead">
            Respuestas directas a las dudas y situaciones más comunes durante el uso del sistema.
          </p>

          <div class="faq-list">
            <details class="faq-item" open>
              <summary>¿Por qué el sistema dice que un producto "ya existe" si no lo veo en la lista?</summary>
              <div class="faq-body">
                Probablemente ese producto fue <strong>archivado</strong> anteriormente.
                Para recuperarlo, ve a la sección <em>Productos</em>, activa la casilla <strong>"Archivados"</strong>
                en la barra superior y pulsa <strong>"Restaurar"</strong>. También puedes hacer clic directamente en
                el botón <em>"Ver registro existente (archivado)"</em> que aparece debajo del aviso de error.
              </div>
            </details>

            <details class="faq-item">
              <summary>¿Por qué no puedo cambiar el factor de conversión o la unidad base de un producto?</summary>
              <div class="faq-body">
                Porque el producto ya posee movimientos asentados en su historial. Alterar un factor
                retroactivamente falsearía los cálculos y saldos de las compras y salidas anteriores. Si el
                fabricante cambió el formato de empaque, añade una <strong>nueva presentación</strong>
                (ej. "Caja x 10") y utiliza esa nueva presentación a partir de ese momento.
              </div>
            </details>

            <details class="faq-item">
              <summary>¿Por qué el botón de archivar producto está deshabilitado?</summary>
              <div class="faq-body">
                Por seguridad contable, un producto <strong>solo se puede archivar si su stock es exactamente 0.00</strong>.
                Esto evita que quede mercadería física olvidada en el almacén sin control. Realiza una Salida
                o un Ajuste de inventario para llevar el stock a cero y el botón de archivar se habilitará al instante.
              </div>
            </details>

            <details class="faq-item">
              <summary>¿Puedo eliminar un movimiento si me equivoqué en la cantidad o producto?</summary>
              <div class="faq-body">
                Lo correcto es <strong>Anular el movimiento</strong> haciendo clic en el icono de papelera en la
                tabla de Movimientos e ingresando el motivo; el stock se revertirá automáticamente y podrás
                asentar el movimiento correcto. Si necesitas purgar el historial, un administrador puede usar
                <em>Configuración &gt; Avanzado &gt; "Eliminar movimientos"</em> con contraseña: esa purga es
                irreversible y no modifica el stock.
              </div>
            </details>

            <details class="faq-item">
              <summary>¿Por qué el sistema me impide anular una Entrada de mercadería?</summary>
              <div class="faq-body">
                Anular una Entrada requiere restar ese stock de las existencias actuales. Si después de esa
                Entrada ya realizaste Salidas o ventas que consumieron esa mercadería, el saldo caería por
                debajo de cero. El sistema bloquea la anulación para proteger la regla de stock no negativo.
              </div>
            </details>

            <details class="faq-item">
              <summary>¿Cómo traslado mi inventario a otra computadora?</summary>
              <div class="faq-body">
                <ol>
                  <li>En la máquina actual, abre <strong>Configuración (⚙)</strong> &gt; <em>Avanzado</em> y pulsa <strong>"Crear respaldo"</strong>.</li>
                  <li>Guarda el archivo <code>.db</code> en una memoria USB o disco externo.</li>
                  <li>Instala la aplicación en la nueva computadora.</li>
                  <li>Abre <strong>Configuración (⚙)</strong> &gt; <strong>"Restaurar respaldo"</strong> y selecciona tu archivo.</li>
                </ol>
              </div>
            </details>
          </div>
        </div>
      {/if}
    </article>
  </div>

  {#snippet footer()}
    <div class="modal-footer-content">
      <span class="footer-hint">Acceso rápido: presiona <strong>F1</strong> en cualquier momento para abrir o cerrar esta ayuda.</span>
      <div class="footer-action">
        <Button variant="secondary" icon={Check} onclick={() => modalService.close()}>
          Entendido
        </Button>
      </div>
    </div>
  {/snippet}
</Modal>

<style>
  .help-container {
    display: flex;
    flex: 1;
    min-height: 0;
    height: 100%;
    gap: 1.5rem;
    overflow: hidden;
  }

  /* Barra lateral del manual */
  .help-nav {
    width: 270px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    border-right: 1px solid var(--border);
    padding-right: 1.25rem;
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.4rem 0.6rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    color: var(--text-muted);
    margin-bottom: 0.5rem;
    flex-shrink: 0;
  }

  .search-input {
    flex: 1;
    border: none;
    outline: none;
    background: transparent;
    font-size: 0.8125rem;
    color: var(--text);
    min-width: 0;
  }

  .nav-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
    min-height: 0;
    overflow-y: auto;
  }

  .nav-empty {
    font-size: 0.8125rem;
    color: var(--text-muted);
    padding: 0.5rem 0.625rem;
  }

  .nav-tab {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.5rem 0.625rem;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    text-align: left;
    transition: background-color 0.15s, color 0.15s;
    width: 100%;
    white-space: nowrap;
  }

  .nav-tab:hover {
    background: var(--surface-2);
    color: var(--text);
  }

  .nav-tab.active {
    background: var(--primary-soft);
    color: var(--primary);
    font-weight: 600;
  }

  .tab-label {
    flex: 1;
    white-space: nowrap;
  }

  .tab-badge {
    font-size: 0.6875rem;
    padding: 0.1rem 0.35rem;
    border-radius: 4px;
    background: var(--surface-3);
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .nav-footer {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--border);
    font-size: 0.75rem;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .version-label {
    font-weight: 600;
  }

  .sub-label {
    font-size: 0.7rem;
    opacity: 0.8;
  }

  /* Contenido principal */
  .help-content {
    flex: 1;
    min-width: 0;
    min-height: 0;
    height: 100%;
    overflow-y: auto;
    padding-right: 0.75rem;
  }

  .topic {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .topic h2 {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 1.2rem;
    color: var(--text);
  }

  :global(.topic-icon) {
    color: var(--primary);
  }

  .topic h3 {
    font-size: 0.95rem;
    margin-top: 0.5rem;
    color: var(--text);
  }

  .lead {
    font-size: 0.9375rem;
    line-height: 1.55;
    color: var(--text);
  }

  .card-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
    gap: 0.75rem;
  }

  .info-card {
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    padding: 0.875rem;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .info-card h3 {
    margin: 0;
    font-size: 0.875rem;
  }

  .info-card p {
    font-size: 0.8125rem;
    color: var(--text-muted);
    line-height: 1.45;
  }

  :global(.accent-icon) {
    color: var(--primary);
  }

  .kpi-demo {
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 0.75rem;
    background: var(--surface);
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .kpi-title {
    font-weight: 600;
    font-size: 0.875rem;
    color: var(--primary);
  }

  .kpi-demo.tone-danger .kpi-title {
    color: var(--danger);
  }

  .kpi-desc {
    font-size: 0.78125rem;
    color: var(--text-muted);
  }

  .feature-list {
    margin: 0;
    padding-left: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    font-size: 0.84375rem;
    color: var(--text);
  }

  .shortcuts-note {
    margin-top: 0.5rem;
    font-size: 0.78125rem;
    color: var(--text-muted);
  }

  .tip-box,
  .warning-box {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border-radius: var(--radius-sm);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .tip-box {
    background: var(--primary-soft);
    border: 1px solid var(--border);
    color: var(--text);
  }

  .tip-box :global(svg) {
    color: var(--primary);
    flex-shrink: 0;
    margin-top: 0.125rem;
  }

  .warning-box {
    background: var(--warning-soft);
    border: 1px solid var(--warning);
    color: var(--text);
  }

  .warning-box :global(svg) {
    color: var(--warning);
    flex-shrink: 0;
    margin-top: 0.125rem;
  }

  .help-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.8125rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    overflow: hidden;
  }

  .help-table th,
  .help-table td {
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid var(--border);
    text-align: left;
  }

  .help-table th {
    background: var(--surface-2);
    font-weight: 600;
    color: var(--text-muted);
  }

  .help-table tr:last-child td {
    border-bottom: none;
  }

  /* Preguntas frecuentes */
  .faq-list {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
  }

  .faq-item {
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    overflow: hidden;
  }

  .faq-item summary {
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    color: var(--text);
    user-select: none;
    transition: background-color 0.15s;
  }

  .faq-item summary:hover {
    background: var(--surface-3);
  }

  .faq-body {
    padding: 0.75rem 1rem 1rem;
    font-size: 0.8125rem;
    line-height: 1.55;
    color: var(--text-muted);
    border-top: 1px solid var(--border);
    background: var(--surface);
  }

  .faq-body ol {
    margin: 0.5rem 0 0;
    padding-left: 1.25rem;
  }

  code {
    background: var(--surface-3);
    padding: 0.125rem 0.35rem;
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.8125rem;
  }

  .modal-footer-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    gap: 1.5rem;
  }

  .footer-hint {
    font-size: 0.8125rem;
    color: var(--text-muted);
    line-height: 1.4;
    flex: 1;
    min-width: 0;
  }

  .footer-action {
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  @media (max-width: 650px) {
    .help-container {
      flex-direction: column;
    }
    .help-nav {
      width: 100%;
      border-right: none;
      border-bottom: 1px solid var(--border);
      padding-right: 0;
      padding-bottom: 0.75rem;
    }
    .nav-list {
      flex-direction: row;
      overflow-x: auto;
    }
  }
</style>
