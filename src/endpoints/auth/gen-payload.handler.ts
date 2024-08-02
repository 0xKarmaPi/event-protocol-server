import crypto from "crypto"
import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import { OtpRepository } from "@root/repositories/otp.repository.js"
import { PAYLOAD_TTL, SHARED_SECRET } from "@root/shared/env.js"
const handler: FastifyPluginAsyncZod = async self => {
  self.get(
    "/generate-payload",
    {
      schema: {
        tags: ["Payload"],
        querystring: z.object({
          email: z.string().email()
        })
      }
    },
    async ({ query }) => {
      const email = query.email
      const now = new Date()
      const expirerIn = new Date(now.getTime() + 15 * 60000)
      const randomBits = crypto.randomBytes(8)

      const currentTime = Math.floor(Date.now() / 1000)
      const expirationTime = Buffer.alloc(8)
      expirationTime.writeBigUint64BE(BigInt(currentTime + Number(PAYLOAD_TTL)))
      const payload = Buffer.concat([randomBits, expirationTime])
      const hmac = crypto.createHmac("sha256", SHARED_SECRET)
      hmac.update(payload)
      const signature = hmac.digest()
      const finalPayload = Buffer.concat([payload, signature])
      const payloadHex = finalPayload.subarray(0, 32).toString("hex")

      await OtpRepository.create({ email, expirerIn, type: "PAYLOAD" })

      return payloadHex
    }
  )
}

export default handler
