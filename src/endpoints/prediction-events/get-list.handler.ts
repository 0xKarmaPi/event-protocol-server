import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"

import { PredictionEventRepository } from "@root/repositories/prediction-event.repository.js"

const predictionEventHandler: FastifyPluginAsyncZod = async self => {
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
  )
}

export default predictionEventHandler
