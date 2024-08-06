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
          description: z
            .string()
            .max(250, "Description is maximum 250 characters")
            .trim(),
          endTime: z
            .string()
            .datetime({ message: "Invalid datetime string! Must be UTC." })
            .refine(data => {
              const endTime = new Date(data)
              const now = new Date()
              return endTime.getTime() > now.getTime()
            }, "End time must be greater than now!")
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
