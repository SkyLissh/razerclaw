import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";
import { deviceDetailQuery, updatePollRateMutation } from "../queries";

export function createDeviceDetail(serial: () => string) {
  const queryClient = useQueryClient();

  const device = createQuery(() => deviceDetailQuery(serial()));
  const updatePollRate = createMutation(() => updatePollRateMutation(queryClient));

  return {
    device,
    updatePollRate,
    supportsPollRate() {
      return device.data?.capabilities?.includes("poll_rate") ?? false;
    },
    hasDpi() {
      return device.data?.capabilities?.includes("dpi") ?? false;
    },
    hasBrightness() {
      return device.data?.capabilities?.includes("brightness") ?? false;
    },
    hasPower() {
      return device.data?.capabilities?.includes("power") ?? false;
    },
  };
}
