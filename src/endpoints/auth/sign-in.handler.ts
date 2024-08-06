import bs58 from "bs58"
import crypto from "crypto"
import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import nacl from "tweetnacl"
import { z } from "zod"

import { OtpRepository } from "@root/repositories/otp.repository.js"
import { UserRepository } from "@root/repositories/user.repository.js"
import { SHARED_SECRET } from "@root/shared/env.js"

const handler: FastifyPluginAsyncZod = async self => {
  //Sig in by Verify 2 conditions:
  // 1. Payload before
  // 2. Valid proof from wallet
  // Verify successfully => Sign in
  self.post(
    "/sign-in",
    {
      schema: {
        tags: ["Auth"],
        body: z.object({
          proof: z.string(),
          signature: z.string(),
          address: z.string()
        })
      }
    },
    // Doing
    async ({ body }, reply) => {
      const { proof, signature, address } = body

      // if (address === "admin") {
      //   const user = await UserRepository.findByAddress(
      //     "5iyhohr3BUG8yKmfydasg9qpqFdZY4fWStdab2WTN5hrrjDDD"
      //   )
      //   if (!user) throw reply.notAcceptable()
      //   const accessToken = self.jwt.sign({
      //     id: user.id
      //   })
      //   return {
      //     accessToken,
      //     user
      //   }
      // }

      // === VERIFY PAYLOAD
      const hexProof = Buffer.from(proof, "hex")
      if (hexProof.length !== 32) {
        throw reply.badRequest("Invalid payload length")
      }
      const mac = crypto.createHmac("sha256", SHARED_SECRET)
      mac.update(hexProof.subarray(0, 16))
      const payloadSignatureBytes = mac.digest()
      const isValidSignature = hexProof
        .subarray(16)
        .equals(payloadSignatureBytes.subarray(0, 16))

      const now = Math.floor(Date.now() / 1000)
      // check payload expiration
      const expireBytes = hexProof.subarray(8, 16)
      const expireTime = expireBytes.readBigUint64BE()
      if (BigInt(now) > expireTime) {
        throw reply.badRequest("Payload expired")
      }
      if (!isValidSignature) {
        throw reply.badRequest("Invalid payload signature")
      }

      const otp = await OtpRepository.findByAddress(address)
      if (!otp) {
        throw reply.notFound("Payload is not existed")
      }
      if (proof !== otp.value) {
        throw reply.badRequest("Wrong payload!")
      }

      // === VERIFY PROOF FROM WALLET
      try {
        const isVerified = nacl.sign.detached.verify(
          new TextEncoder().encode(proof),
          bs58.decode(signature),
          bs58.decode(address)
        )
        if (!isVerified) {
          throw reply.badRequest("Verify failed!")
        }
      } catch (error) {
        throw reply.badRequest(String(error))
      }

      // ==== SIGN IN
      const user = await UserRepository.findByAddress(body.address)

      if (user) {
        const accessToken = self.jwt.sign({
          id: user.id
        })
        return {
          accessToken,
          user
        }
      }

      // New user register
      const randomString = (Math.random() + 1).toString(36).substring(5)
      const username = "user-" + randomString
      const newUser = await UserRepository.create({ ...body, username })
      const accessToken = self.jwt.sign({
        id: newUser.id
      })
      return {
        accessToken,
        user: newUser
      }
    }
  )
}

export default handler
