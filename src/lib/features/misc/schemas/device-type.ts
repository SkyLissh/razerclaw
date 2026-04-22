import * as z from "zod";

export const DeviceTypeSchema = z.enum(["mouse", "keyboard", "headset", "other"]);

export type DeviceType = z.infer<typeof DeviceTypeSchema>;
