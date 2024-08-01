import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import { PredictionEventRepository } from "@root/repositories/prediction-event.repository.js"

const predictionEventHandler: FastifyPluginAsyncZod = async self => {
  // Get all
  self.get(
    "/",
    {
      schema: {
        tags: ["Prediction Event"]
      }
    },
    async () => {
      return PredictionEventRepository.findAll()
    }
  ),
    // Create new
    self.post(
      "/",
      {
        schema: {
          tags: ["Prediction Event"],
          body: z.object({
            title: z
              .string()
              .max(500, "Title is maximum 500 characters")
              .trim(),
            balance: z.string().trim().default("0"),
            endTime: z
              .string()
              .datetime({ message: "Invalid datetime string! Must be UTC." })
          })
        }
      },
      async ({ body }) => {
        return PredictionEventRepository.create(body)
      }
    ),
    // Get detail
    self.get(
      "/:id",
      {
        schema: {
          tags: ["Prediction Event"],
          params: z.object({
            id: z.string()
          })
        }
      },
      async ({ params }, reply) => {
        const predictionEvent = await PredictionEventRepository.findDetailById(
          +params.id
        )
        if (!predictionEvent) {
          return reply
            .status(406)
            .send({ message: "Not found prediction event!" })
        }
        return predictionEvent
      }
    ),
    // Delete by id
    self.delete(
      "/:id",
      {
        schema: {
          tags: ["Prediction Event"],
          params: z.object({
            id: z.string()
          })
        }
      },
      async ({ params }, reply) => {
        const id = +params.id
        // Check exist event
        const predictionEvent = await PredictionEventRepository.findById(id)
        if (!predictionEvent)
          return reply
            .status(406)
            .send({ message: "Not found prediction event!" })

        return PredictionEventRepository.deleteById(id)
      }
    )
}

export default predictionEventHandler
