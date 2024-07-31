import z from "zod"

export const paginationParams = () =>
  z.object({
    page: z
      .string()
      .optional()
      .default("1")
      .transform(Number)
      .pipe(z.number().int().min(1)),
    take: z
      .string()
      .optional()
      .default("60")
      .transform(Number)
      .pipe(z.number().int().positive().max(300))
  })
