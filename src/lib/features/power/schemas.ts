import * as z from "zod";

export const IdleTimeSchema = z.int().nonnegative();
export const LowBatteryThresholdSchema = z.int().nonnegative();

export const PowerSchema = z.object({
  battery: z.number().transform((value) => parseFloat(value.toFixed(0))),
  idle_time: IdleTimeSchema,
  low_battery_threshold: LowBatteryThresholdSchema,
  is_charging: z.boolean(),
});

export type Power = z.infer<typeof PowerSchema>;
