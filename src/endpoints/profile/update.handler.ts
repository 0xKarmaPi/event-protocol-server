import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import authPlugin from "@root/plugins/auth.plugin.js"
import { UserRepository } from "@root/repositories/user.repository.js"
import { SECURITY } from "@root/shared/constant.js"

const ProfileHandler: FastifyPluginAsyncZod = async self => {
  self.register(authPlugin).patch(
    "/",
    {
      schema: {
        tags: ["Profile"],
        security: SECURITY,
        body: z.object({
          bio: z.string().max(500, "Bio is maximum 500 characters").trim()
        })
      }
    },
    async ({ body, user }) => {
      return UserRepository.updateById(user.id, body)
    }
  )
}

export default ProfileHandler
