<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  import { createDevicesList } from "$lib/features/misc/stores";

  const { devices } = createDevicesList();

  $effect(() => {
    if (devices.data) {
      goto(resolve("/devices/[serial]", { serial: devices.data?.[0]?.serial ?? "" }));
    }
  });
</script>
