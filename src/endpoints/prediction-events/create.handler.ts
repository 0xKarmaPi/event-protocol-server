import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import authPlugin from "@root/plugins/auth.plugin.js"
import { PredictionEventRepository } from "@root/repositories/prediction-event.repository.js"
import { UserRepository } from "@root/repositories/user.repository.js"
import { SECURITY } from "@root/shared/constant.js"

const predictionEventHandler: FastifyPluginAsyncZod = async self => {
  self.register(authPlugin).post(
    "/",
    {
      schema: {
        tags: ["Prediction Event"],
        security: SECURITY,
        body: z.object({
          title: z.string().max(500, "Title is maximum 500 characters").trim(),
          balance: z.string().trim().default("0"),
          endTime: z
            .string()
            .datetime({ message: "Invalid datetime string! Must be UTC." })
        })
      }
    },
    async ({ body, user }, reply) => {
      const userId = user.id
      const existUser = await UserRepository.findById(userId)
      if (!existUser) {
        throw reply.notFound("Not found user")
      }
      return PredictionEventRepository.create(userId, body)
    }
  )
}

export default predictionEventHandler
