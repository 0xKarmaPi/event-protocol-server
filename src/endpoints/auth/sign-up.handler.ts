import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import { UserRepository } from "@root/repositories/user.repository.js"

const handler: FastifyPluginAsyncZod = async self => {
  self.post(
    "/sign-up",
    {
      schema: {
        tags: ["Auth"],
        body: z.object({
          username: z.string().min(2, "User name is minimum 2 characters"),
          email: z.string().email("Invalid email"),
          hashedCode: z.string()
        })
      }
    },
    async ({ body }, reply) => {
      const existEmail = await UserRepository.findByEmail(body.email)
      if (existEmail) throw reply.badRequest("Email is existed")

      const existHashedCode = await UserRepository.findByHashedCode(
        body.hashedCode
      )
      if (existHashedCode) throw reply.badRequest("Hashed code is existed")
      return UserRepository.create(body)
    }
  )
}

export default handler
