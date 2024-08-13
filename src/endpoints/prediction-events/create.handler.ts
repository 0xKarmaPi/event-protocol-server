import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import authPlugin from "@root/plugins/auth.plugin.js"
import { EventOptionRepository } from "@root/repositories/prediction-event-answer.repository.js"
import { EventRepository } from "@root/repositories/prediction-event.repository.js"
import { UserRepository } from "@root/repositories/user.repository.js"
import { SECURITY } from "@root/shared/constant.js"

const EventHandler: FastifyPluginAsyncZod = async self => {
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
            }, "End time must be greater than now!"),
          options: z
            .array(
              z.object({
                description: z
                  .string()
                  .max(100, "Description is maximum 100 characters")
                  .trim(),
                token: z
                  .string()
                  .max(100, "Token is maximum 100 characters")
                  .trim(),
                address: z.string().toLowerCase().trim().optional(),
                isCorrect: z.boolean()
              })
            )
            .length(2, "Options must contain exactly 2 items")
        })
      }
    },
    async ({ body, user }, reply) => {
      const { options, ...payload } = body
      const userId = user.id
      const existUser = await UserRepository.findById(userId)
      if (!existUser) {
        throw reply.notFound("Not found user")
      }
      const createdEvent = await EventRepository.create(userId, payload)

      // Check duplicate correct or incorrect answer: isCorrect, value
      const isDuplicatedAnswer =
        // options[0].isCorrect === options[1].isCorrect ||
        options[0].description === options[1].description
      if (isDuplicatedAnswer) {
        await EventRepository.delete(userId, createdEvent.id)
        throw reply.notAcceptable(`Duplicated value or correct answer!`)
      }

      for (const option of options) {
        await EventOptionRepository.create(createdEvent.id, option)
      }

      return EventRepository.findDetailById(createdEvent.id)
    }
  )
}

export default EventHandler
