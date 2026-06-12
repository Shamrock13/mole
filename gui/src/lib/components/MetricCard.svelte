<script>
  import Sparkline from "./Sparkline.svelte";

  // One floating glass tile on the STATUS grid. `active` drives the
  // gentle pulse dot; `detail` is the secondary line under the value.
  let {
    title,
    value,
    detail = "",
    history = [],
    max = 100,
    min = 0,
    active = false,
    children,
  } = $props();
</script>

<article class="glass card" aria-label={`${title}: ${value}`}>
  <header>
    <h3 class="caption">{title}</h3>
    {#if active}
      <span class="dot pulse" aria-hidden="true"></span>
    {/if}
  </header>

  <p class="value mono">{value}</p>
  {#if detail}
    <p class="detail">{detail}</p>
  {/if}

  <div class="spark glass-inset">
    <Sparkline values={history} {max} {min} label={title} width={200} height={30} />
  </div>

  {#if children}
    {@render children()}
  {/if}
</article>

<style>
  .card {
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    min-width: 0;
    transition: transform var(--duration-fast) var(--ease-out);
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .dot {
    width: 7px;
    height: 7px;
    border-radius: var(--radius-pill);
    background: var(--accent);
    box-shadow: 0 0 8px var(--accent-glow);
  }
  .value {
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-tight);
  }
  .detail {
    font-size: var(--text-xs);
    color: var(--ink-secondary);
    margin-top: calc(var(--space-1) * -1);
  }
  .spark {
    padding: var(--space-2) var(--space-3);
    margin-top: auto;
  }
  .spark :global(svg) {
    width: 100%;
    height: 30px;
  }
</style>
