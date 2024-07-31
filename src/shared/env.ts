import { parseEnv } from "util"
import z from "zod"

import { decryptEnv } from "@root/helpers/decrypt-env.js"

const schema = z.object({
  DATABASE_URL: z.string().min(1)
})

export const { DATABASE_URL } = schema.parse(
  process.env.LOCAL
    ? process.env
    : parseEnv(decryptEnv(process.env.SECRET_ENC!))
)
