<script lang="ts">
  import { m } from "$lib/paraglide/messages";

  import { Keyboard, Mouse, RefreshCcw } from "@lucide/svelte";

  import { resolve } from "$app/paths";
  import { Button } from "$lib/components/ui/button";
  import type { DeviceType } from "$lib/schemas/device";

  type Props = {
    devices: { name: string; serial: string; type: DeviceType }[];
    selected: string | null;
    onSelect: (serial: string) => void;
  };

  const { devices, selected, onSelect }: Props = $props();
</script>

<aside class="h-full space-y-4 bg-card p-4">
  <div class="flex items-center justify-between">
    <h2>{m.devices()}</h2>

    <Button variant="ghost" size="icon">
      <RefreshCcw />
    </Button>
  </div>
  <ul class="space-y-2">
    {#each devices as device (device.serial)}
      <li>
        <Button
          href={resolve("/devices/[serial]", { serial: device.serial })}
          data-selected={selected === device.serial}
          variant="ghost"
          class="h-auto w-full justify-start py-2 data-[selected=true]:text-primary data-[selected=true]:hover:bg-primary/10"
          onclick={() => onSelect(device.serial)}
        >
          {#if device.type === "mouse"}
            <Mouse class="size-5" />
          {:else if device.type === "keyboard"}
            <Keyboard class="size-5" />
          {:else}
            <span>Unknown Device</span>
          {/if}
          <p class="font-medium">{device.name}</p>
        </Button>
      </li>
    {/each}
  </ul>
</aside>
