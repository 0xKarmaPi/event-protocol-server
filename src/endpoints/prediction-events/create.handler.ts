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
          userId: z.number(),
          title: z.string().max(500, "Title is maximum 500 characters").trim(),
          balance: z.string().trim().default("0"),
          endTime: z
            .string()
            .datetime({ message: "Invalid datetime string! Must be UTC." })
        })
      }
    },
    async ({ body }, reply) => {
      const { userId, ...payload } = body
      const user = await UserRepository.findById(+userId)
      if (!user) {
        return reply.status(404).send("Not found user")
      }
      return PredictionEventRepository.create(userId, payload)
    }
  )
}

export default predictionEventHandler
