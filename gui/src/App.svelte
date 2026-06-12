<script>
  import Sidebar from "./lib/components/Sidebar.svelte";
  import Status from "./modules/Status.svelte";
  import Clean from "./modules/Clean.svelte";
  import Software from "./modules/Software.svelte";
  import Optimize from "./modules/Optimize.svelte";
  import Analyze from "./modules/Analyze.svelte";

  const MODULES = [
    { id: "status", title: "Status", tagline: "Every heartbeat, in its light.", icon: "◉", component: Status },
    { id: "clean", title: "Clean", tagline: "Rainwater clears the soil.", icon: "❋", component: Clean },
    { id: "software", title: "Software", tagline: "Red dust covers what you've outgrown.", icon: "▦", component: Software },
    { id: "optimize", title: "Optimize", tagline: "Closest orbit, swiftest run.", icon: "◎", component: Optimize },
    { id: "analyze", title: "Analyze", tagline: "Widest eye, smallest folder on the map.", icon: "▤", component: Analyze },
  ];

  let current = $state("status");
  let CurrentView = $derived(MODULES.find((m) => m.id === current).component);

  // Cmd+1…5 jumps between modules, mirroring native macOS apps.
  function onKeydown(event) {
    if (!event.metaKey) return;
    const index = Number(event.key) - 1;
    if (index >= 0 && index < MODULES.length) {
      event.preventDefault();
      current = MODULES[index].id;
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

<div class="shell">
  <Sidebar modules={MODULES} {current} onNavigate={(id) => (current = id)} />
  <main class="stage" aria-live="off">
    <CurrentView />
  </main>
</div>

<style>
  .shell {
    display: flex;
    height: 100%;
  }
  .stage {
    flex: 1;
    min-width: 0;
    overflow-y: auto;
    padding: var(--space-5);
  }
</style>
