<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  import { createDevices } from "$lib/composables/devices";

  const { devices } = createDevices();

  $effect(() => {
    if (devices.data) {
      goto(resolve("/devices/[serial]", { serial: devices.data?.[0]?.serial ?? "" }));
    }
  });
</script>
