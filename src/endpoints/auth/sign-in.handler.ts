import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import { UserRepository } from "@root/repositories/user.repository.js"

const handler: FastifyPluginAsyncZod = async self => {
  self.post(
    "/sign-in",
    {
      schema: {
        tags: ["Auth"],
        body: z.object({
          email: z.string().email("Invalid email"),
          hashedCode: z.string()
        })
      }
    },
    async ({ body }, reply) => {
      const user = await UserRepository.findByEmailAndHashedCode(body)
      if (!user) {
        throw reply.badRequest("Not found user")
      }
      const accessToken = self.jwt.sign({
        payload: user
      })
      return {
        accessToken,
        user
      }
    }
  )
}

export default handler
