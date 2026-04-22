import { createCoalescedExecutor } from "$lib/stores";
import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";
import { throttle } from "es-toolkit";
import { brightnessQuery, updateBrightnessMutation } from "./queries";

export function createBrightness(serial: () => string) {
  const queryClient = useQueryClient();

  const brightness = createQuery(() => brightnessQuery(serial()));
  const mutation = createMutation(() => updateBrightnessMutation(queryClient, serial()));

  const coalesced = createCoalescedExecutor(async (value: number) => {
    await mutation.mutateAsync(value);
  });

  const updateBrightness = throttle((value: number) => {
    coalesced(value);
  }, 50);

  return {
    brightness,
    updateBrightness,
  };
}
