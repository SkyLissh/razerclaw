import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";

import { deviceBySerialResource } from "$lib/resources/razer/device-by-serial";
import { getServicesContext } from "$lib/services";

export function createDeviceBySerial(serial: () => string) {
  const { razer } = getServicesContext();
  const queryClient = useQueryClient();

  const resource = deviceBySerialResource(razer, serial);

  const device = createQuery(() => resource.getDeviceBySerial());
  const updatePollRate = createMutation(() => resource.updatePollRate(queryClient));

  return {
    device,
    updatePollRate,
  };
}
