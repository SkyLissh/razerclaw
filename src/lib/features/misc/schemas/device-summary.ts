import * as z from "zod";

import { DeviceCapabilitySchema } from "./device-capability";
import { DeviceTypeSchema } from "./device-type";

export const DeviceSummarySchema = z.object({
  name: z.string(),
  serial: z.string(),
  type: DeviceTypeSchema,
  image: z.string(),
  mode: z.string(),
  driver_version: z.string(),
  firmware: z.string(),
  matrix_dimensions: z.array(z.int()).nullable(),
  poll_rate: z.int().nullable(),
  razer_urls: z.string(),
  supported_poll_rates: z.array(z.int()).nullable(),
  vid: z.int(),
  pid: z.int(),
  has_dedicated_macro_keys: z.boolean(),
  has_matrix: z.boolean(),
  capabilities: z.array(DeviceCapabilitySchema),
});

export type DeviceSummary = z.infer<typeof DeviceSummarySchema>;
