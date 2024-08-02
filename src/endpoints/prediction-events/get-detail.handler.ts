import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import { PredictionEventRepository } from "@root/repositories/prediction-event.repository.js"

const predictionEventHandler: FastifyPluginAsyncZod = async self => {
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
  )
}

export default predictionEventHandler
