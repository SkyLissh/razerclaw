import * as z from "zod";

export const DeviceType = z.enum(["keyboard", "mouse", "headset", "mousepad", "other"]);

export type DeviceType = z.infer<typeof DeviceType>;

export const DeviceSummary = z.object({
  name: z.string(),
  type: DeviceType,
  serial: z.string(),
  image: z.string(),
  driver_version: z.string(),
  firmware: z.string(),
});

export type DeviceSummary = z.infer<typeof DeviceSummary>;

export const DevicePower = z.object({
  battery: z.number(),
  idle_time: z.number(),
  low_battery_threshold: z.number(),
  is_charging: z.boolean(),
});

export type DevicePower = z.infer<typeof DevicePower>;

const DeviceDPIStages = z.object({
  active: z.number(),
  stages: z.array(z.object({ x: z.number(), y: z.number() })),
});

export const DeviceDPI = z.object({
  dpi: z.object({ x: z.number(), y: z.number() }),
  stages: DeviceDPIStages,
  max_dpi: z.number(),
});

export type DeviceDPI = z.infer<typeof DeviceDPI>;

export const DeviceDetail = DeviceSummary.extend({
  poll_rate: z.number().nullable(),
  supported_poll_rates: z.array(z.number()).nullable(),
  mode: z.string().nullable(),
  matrix_dimensions: z.array(z.number()).nullable(),
  razer_urls: z.string().nullable(),
  has_dedicated_macro_keys: z.boolean(),
  has_matrix: z.boolean(),
  vid: z.number(),
  pid: z.number(),
  firmware: z.string(),

  // DPI properties
  dpi: DeviceDPI.nullable(),

  // Power properties
  power: DevicePower.nullable(),

  // Brightness properties
  brightness: z.number().nullable(),

  // Gamemode properties
  gamemode_enabled: z.boolean().nullable(),
});

export type DeviceDetail = z.infer<typeof DeviceDetail>;
