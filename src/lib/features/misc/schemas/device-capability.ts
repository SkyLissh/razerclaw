import * as z from "zod";

export const DeviceCapabilitySchema = z.enum([
  "dpi",
  "power",
  "brightness",
  "gamemode",
  "poll_rate",
]);

export type DeviceCapability = z.infer<typeof DeviceCapabilitySchema>;
