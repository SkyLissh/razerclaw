import { queryOptions } from "@tanstack/svelte-query";

import type { RazerService } from "$lib/services/razer";
import { razerQueryKeys } from "./keys";

export function devicesResource(service: RazerService) {
  return {
    getDevices: () =>
      queryOptions({
        queryKey: razerQueryKeys.devices,
        queryFn: () => service.getDevices(),
      }),
  };
}
