import * as z from "zod";

export const BrightnessSchema = z.object({
  brightness: z.number().min(0).max(100),
});

export type Brightness = z.infer<typeof BrightnessSchema>;
