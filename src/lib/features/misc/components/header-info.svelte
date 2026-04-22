<script lang="ts">
  import { m } from "$lib/paraglide/messages";

  import { Button } from "$lib/components/ui/button";
  import * as Tooltip from "$lib/components/ui/tooltip";

  import { Files } from "@lucide/svelte";
  import type { Snippet } from "svelte";
  import { createCopySerial } from "../stores/create-copy-serial.svelte";

  type Props = {
    name: string;
    serial: string;
    image: string;
    children?: Snippet;
  };

  const { name, serial, image, children }: Props = $props();

  const copySerial = createCopySerial();
</script>

<div class="flex items-center justify-between gap-4">
  <div>
    <h2 class="text-2xl font-bold">{name}</h2>

    <div class="flex items-center gap-1">
      <p class="text-sm text-muted-foreground">{serial}</p>
      <Tooltip.Root disableCloseOnTriggerClick>
        <Tooltip.Trigger>
          <Button variant="ghost" size="icon" onclick={() => copySerial.copy(serial)}>
            <span class="sr-only">{m.copy_to_clipboard()}</span>
            <Files class="text-muted-foreground" />
          </Button>
        </Tooltip.Trigger>
        <Tooltip.Content side="bottom">
          <p class="text-xs">
            {#if copySerial.state === "copied"}
              {m.copied()}
            {:else if copySerial.state === "error"}
              {m.copy_failed()}
            {:else}
              {m.copy_to_clipboard()}
            {/if}
          </p>
        </Tooltip.Content>
      </Tooltip.Root>
    </div>

    {@render children?.()}
  </div>
  <div class="relative mr-6 flex items-center justify-center">
    <div class="absolute size-40 rounded-full bg-primary/15 blur-2xl"></div>
    <div class="absolute size-16 rounded-full bg-primary/70 blur-xl"></div>
    <img src={image} alt={name} class="relative size-32 object-contain" />
  </div>
</div>
