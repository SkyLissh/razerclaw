<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";

  import { LineSquiggle, Minus, Square, X } from "@lucide/svelte";

  import { Button } from "$lib/components/ui/button";

  import { m } from "$lib/paraglide/messages";

  const window = getCurrentWindow();
</script>

<div
  class="grid grid-cols-[1fr_auto_1fr] items-center justify-end rounded bg-card px-1 py-0.5"
  data-tauri-drag-region
>
  <div></div>

  <h1
    class="iteitems-center flex gap-2 justify-self-center text-sm font-medium"
    data-tauri-drag-region
  >
    <LineSquiggle data-tauri-drag-region class="text-primary" />
    RazerClaw
  </h1>

  <div class="flex gap-2 justify-self-end">
    <Button
      size="icon"
      variant="ghost"
      class="transition-colors active:translate-y-0"
      onclick={() => window.minimize()}
    >
      <span class="sr-only">{m.minimize()}</span>
      <Minus />
    </Button>

    <Button
      size="icon"
      variant="ghost"
      class="transition-colors active:translate-y-0"
      onclick={() => window.toggleMaximize()}
    >
      <span class="sr-only">{m.maximize()}</span>
      <Square />
    </Button>

    <Button
      class="transition-colors hover:bg-destructive/20 hover:text-destructive active:translate-y-0 aria-expanded:bg-destructive/20 aria-expanded:text-destructive"
      size="icon"
      variant="ghost"
      onclick={() => window.close()}
    >
      <span class="sr-only">{m.close()}</span>
      <X />
    </Button>
  </div>
</div>
