<script>
  import { fanMode, setFanMode } from "../metrics.js";

  // Segmented Auto / Cool / Quiet control, rendered as a proper
  // radiogroup so VoiceOver announces it correctly. Arrow keys move
  // between modes per the WAI-ARIA radio pattern.
  let { supported = true, rpm = 0 } = $props();

  const MODES = [
    { id: "auto", label: "Auto", hint: "System-managed fan curve" },
    { id: "cool", label: "Cool", hint: "Prioritize low temperatures" },
    { id: "quiet", label: "Quiet", hint: "Prioritize low noise" },
  ];

  function onKeydown(event, index) {
    let target = null;
    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      target = MODES[(index + 1) % MODES.length];
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      target = MODES[(index - 1 + MODES.length) % MODES.length];
    }
    if (target) {
      event.preventDefault();
      setFanMode(target.id);
      document.getElementById(`fan-${target.id}`)?.focus();
    }
  }
</script>

{#if supported}
  <div class="fan-control">
    <div
      class="segments glass-inset"
      role="radiogroup"
      aria-label="Fan mode"
    >
      {#each MODES as mode, i (mode.id)}
        <button
          id={`fan-${mode.id}`}
          role="radio"
          aria-checked={$fanMode === mode.id}
          tabindex={$fanMode === mode.id ? 0 : -1}
          title={mode.hint}
          class="segment"
          class:selected={$fanMode === mode.id}
          onclick={() => setFanMode(mode.id)}
          onkeydown={(e) => onKeydown(e, i)}
        >
          {mode.label}
        </button>
      {/each}
    </div>
    <p class="rpm mono" aria-live="polite">{Math.round(rpm)} rpm</p>
  </div>
{:else}
  <p class="unsupported">Fan control isn't available on this Mac.</p>
{/if}

<style>
  .fan-control {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }
  .segments {
    display: inline-flex;
    padding: 2px;
    border-radius: var(--radius-pill);
  }
  .segment {
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-pill);
    font-size: var(--text-xs);
    font-weight: var(--weight-medium);
    color: var(--ink-secondary);
    transition:
      background var(--duration-fast) var(--ease-out),
      color var(--duration-fast) var(--ease-out);
  }
  .segment.selected {
    background: var(--accent);
    color: #fff;
    box-shadow: 0 1px 6px var(--accent-glow);
  }
  .rpm {
    font-size: var(--text-xs);
    color: var(--ink-secondary);
  }
  .unsupported {
    font-size: var(--text-xs);
    color: var(--ink-tertiary);
  }
</style>
