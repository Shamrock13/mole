<script>
  // Minimalist 60-second sparkline. Pure SVG, no chart library:
  // one path per render, hairline stroke, soft area fill.
  let {
    values = [],
    max = 100,
    min = 0,
    label = "history",
    width = 120,
    height = 28,
  } = $props();

  const PAD = 2;

  let points = $derived.by(() => {
    const n = values.length;
    if (n < 2) return "";
    const span = max - min || 1;
    const out = [];
    for (let i = 0; i < n; i++) {
      const v = values[i];
      if (v == null) continue;
      const x = PAD + (i / (n - 1)) * (width - PAD * 2);
      const clamped = Math.min(max, Math.max(min, v));
      const y = height - PAD - ((clamped - min) / span) * (height - PAD * 2);
      out.push(`${x.toFixed(1)},${y.toFixed(1)}`);
    }
    return out.join(" ");
  });

  let areaPath = $derived(
    points
      ? `M ${points.split(" ")[0]} L ${points.replaceAll(" ", " L ")} L ${width - PAD},${height - PAD} L ${PAD},${height - PAD} Z`
      : ""
  );

  let latest = $derived.by(() => {
    for (let i = values.length - 1; i >= 0; i--) {
      if (values[i] != null) return values[i];
    }
    return null;
  });
</script>

<svg
  viewBox={`0 0 ${width} ${height}`}
  {width}
  {height}
  role="img"
  aria-label={`${label}, last 60 seconds${latest != null ? `, currently ${Math.round(latest)}` : ""}`}
  class="sparkline"
>
  {#if areaPath}
    <path d={areaPath} class="area" />
  {/if}
  {#if points}
    <polyline {points} class="line" />
  {/if}
</svg>

<style>
  .sparkline {
    display: block;
    overflow: visible;
  }
  .sparkline:empty {
    opacity: 0.3;
  }
  .area {
    fill: var(--accent-soft);
    stroke: none;
  }
  .line {
    fill: none;
    stroke: var(--accent);
    stroke-width: 1.5;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
</style>
