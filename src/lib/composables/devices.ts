import { createQuery } from "@tanstack/svelte-query";

import { devicesResource } from "$lib/resources/razer/devices";
import { getServicesContext } from "$lib/services";

export function createDevices() {
  const { razer } = getServicesContext();

  const resource = devicesResource(razer);

  const devices = createQuery(() => resource.getDevices());

  return {
    devices,
  };
}
