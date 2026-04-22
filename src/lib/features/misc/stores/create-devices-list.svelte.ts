import { createQuery } from "@tanstack/svelte-query";
import { devicesQuery } from "../queries";

export function createDevicesList() {
  const devices = createQuery(() => devicesQuery());

  return { devices };
}
