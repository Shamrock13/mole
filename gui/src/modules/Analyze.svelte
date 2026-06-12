<script>
  // ANALYZE — "Widest eye, smallest folder on the map."
  // Squarified-ish treemap over mock disk data with drill-down and a
  // context-menu mockup (Reveal in Finder / Move to Trash). The Tauri
  // backend will feed real scan results from cmd/analyze.
  import { formatBytes } from "../lib/format.js";

  const ROOT = {
    name: "Macintosh HD",
    children: [
      { name: "Applications", bytes: 92e9, children: [
        { name: "Xcode.app", bytes: 34e9 },
        { name: "Final Cut Pro.app", bytes: 4.1e9 },
        { name: "Other apps", bytes: 53.9e9 },
      ]},
      { name: "~/Library", bytes: 118e9, children: [
        { name: "Developer", bytes: 46e9 },
        { name: "Caches", bytes: 21e9 },
        { name: "Application Support", bytes: 38e9 },
        { name: "Other", bytes: 13e9 },
      ]},
      { name: "~/Movies", bytes: 64e9 },
      { name: "~/Pictures", bytes: 41e9 },
      { name: "~/Downloads", bytes: 18e9 },
      { name: "System (protected)", bytes: 22e9, protected: true },
    ],
  };

  let path = $state([ROOT]);
  let node = $derived(path[path.length - 1]);
  let menu = $state(null); // { x, y, item }

  // Simple slice-and-dice layout: rows alternate direction by depth.
  // Cheap (O(n)), allocation-free per frame, fine for a mock shell.
  let tiles = $derived.by(() => {
    const children = node.children ?? [];
    const total = children.reduce((sum, child) => sum + child.bytes, 0) || 1;
    let offset = 0;
    return children.map((child) => {
      const frac = child.bytes / total;
      const tile = { item: child, x: offset * 100, w: frac * 100 };
      offset += frac;
      return tile;
    });
  });

  function enter(item) {
    if (item.protected) return;
    if (item.children) path = [...path, item];
  }

  function up() {
    if (path.length > 1) path = path.slice(0, -1);
  }

  function openMenu(event, item) {
    event.preventDefault();
    menu = { x: event.clientX, y: event.clientY, item };
  }
</script>

<svelte:window onclick={() => (menu = null)} />

<section aria-labelledby="analyze-title">
  <header class="page-head">
    <h1 id="analyze-title">Analyze</h1>
    <p class="subtitle">Drill into the largest branches. Deletions from here always route to Trash.</p>
  </header>

  <nav class="crumbs" aria-label="Breadcrumb">
    {#each path as crumb, i (i)}
      <button
        class="crumb"
        disabled={i === path.length - 1}
        onclick={() => (path = path.slice(0, i + 1))}
      >
        {crumb.name}
      </button>
      {#if i < path.length - 1}<span aria-hidden="true">›</span>{/if}
    {/each}
    {#if path.length > 1}
      <button class="crumb up" onclick={up} aria-label="Go up one level">⌃ Up</button>
    {/if}
  </nav>

  <div class="map glass" role="group" aria-label={`Contents of ${node.name}`}>
    {#each tiles as tile (tile.item.name)}
      <button
        class="tile"
        class:protected={tile.item.protected}
        class:branch={tile.item.children}
        style:left={`${tile.x}%`}
        style:width={`calc(${tile.w}% - 4px)`}
        onclick={() => enter(tile.item)}
        oncontextmenu={(e) => openMenu(e, tile.item)}
        aria-label={`${tile.item.name}, ${formatBytes(tile.item.bytes)}${tile.item.protected ? ", protected" : tile.item.children ? ", open folder" : ""}`}
      >
        <span class="tile-name">{tile.item.name}</span>
        <span class="tile-size mono">{formatBytes(tile.item.bytes)}</span>
      </button>
    {/each}
  </div>

  {#if menu}
    <ul
      class="context glass-raised"
      role="menu"
      style:left={`${menu.x}px`}
      style:top={`${menu.y}px`}
    >
      <li role="menuitem"><button>Reveal in Finder</button></li>
      {#if !menu.item.protected}
        <li role="menuitem"><button class="trash">Move to Trash…</button></li>
      {:else}
        <li class="blocked-hint">Protected system path</li>
      {/if}
    </ul>
  {/if}
</section>

<style>
  .page-head { margin-bottom: var(--space-4); }
  h1 { font-size: var(--text-xl); font-weight: var(--weight-bold); }
  .subtitle { font-size: var(--text-sm); color: var(--ink-secondary); margin-top: var(--space-1); }
  .crumbs {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-bottom: var(--space-3);
    color: var(--ink-tertiary);
  }
  .crumb {
    font-size: var(--text-xs);
    font-weight: var(--weight-medium);
    color: var(--ink-secondary);
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
  }
  .crumb:not(:disabled):hover { background: var(--glass-inset); }
  .crumb:disabled { color: var(--ink-primary); cursor: default; }
  .crumb.up { margin-left: auto; color: var(--accent); }
  .map {
    position: relative;
    height: 360px;
    padding: var(--space-2);
  }
  .tile {
    position: absolute;
    top: var(--space-2);
    bottom: var(--space-2);
    margin-left: 2px;
    border-radius: var(--radius-md);
    background: var(--accent-soft);
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    align-items: flex-start;
    gap: 2px;
    padding: var(--space-3);
    overflow: hidden;
    text-align: left;
    transition: background var(--duration-fast) var(--ease-out);
  }
  .tile:hover { background: var(--accent); color: #fff; }
  .tile.protected {
    background: var(--glass-inset);
    color: var(--ink-tertiary);
    cursor: not-allowed;
  }
  .tile.protected:hover { background: var(--glass-inset); color: var(--ink-tertiary); }
  .tile-name {
    font-size: var(--text-xs);
    font-weight: var(--weight-semibold);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }
  .tile-size { font-size: var(--text-xs); opacity: 0.75; }
  .context {
    position: fixed;
    z-index: 20;
    list-style: none;
    padding: var(--space-2);
    min-width: 180px;
  }
  .context button {
    display: block;
    width: 100%;
    text-align: left;
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-sm);
    font-size: var(--text-sm);
  }
  .context button:hover { background: var(--accent); color: #fff; }
  .context .trash:hover { background: var(--danger); }
  .blocked-hint {
    padding: var(--space-2) var(--space-3);
    font-size: var(--text-xs);
    color: var(--ink-tertiary);
  }
</style>
