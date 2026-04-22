import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";

import { dpiQuery, updateDpiMutation } from "./queries";

export function createDeviceDpi(serial: () => string) {
  const queryClient = useQueryClient();

  const dpi = createQuery(() => dpiQuery(serial()));
  const updateDpi = createMutation(() => updateDpiMutation(queryClient));

  return {
    dpi,
    updateDpi,
  };
}
