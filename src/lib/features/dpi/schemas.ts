import * as z from "zod";

export const DpiStageSchema = z.object({
  x: z.int().nonnegative(),
  y: z.int().nonnegative(),
});

export const DpiStagesSchema = z.object({
  active: z.int().nonnegative(),
  stages: z.array(DpiStageSchema).min(1),
});

export const DeviceDpiSchema = z.object({
  dpi: DpiStageSchema,
  stages: DpiStagesSchema,
  max_dpi: z.int(),
});

export type DpiStage = z.infer<typeof DpiStageSchema>;
export type DpiStages = z.infer<typeof DpiStagesSchema>;
export type DeviceDpi = z.infer<typeof DeviceDpiSchema>;
