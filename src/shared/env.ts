import { parseEnv } from "util"
import z from "zod"

import { decryptEnv } from "@root/helpers/decrypt-env.js"

const schema = z.object({
  DATABASE_URL: z.string().min(1),
  JWT_SECRET_KEY: z.string().min(1),
  PAYLOAD_TTL: z.string().min(1),
  SHARED_SECRET: z.string().min(1)
})

export const { DATABASE_URL, JWT_SECRET_KEY, PAYLOAD_TTL, SHARED_SECRET } =
  schema.parse(
    process.env.LOCAL
      ? process.env
      : parseEnv(decryptEnv(process.env.SECRET_ENC!))
  )
