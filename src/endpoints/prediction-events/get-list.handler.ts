import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import { PredictionEventRepository } from "@root/repositories/prediction-event.repository.js"

const predictionEventHandler: FastifyPluginAsyncZod = async self => {
  self.get(
    "/",
    {
      schema: {
        tags: ["Prediction Event"],
        querystring: z.object({
          page: z
            .string()
            .min(1, "Invalid page")
            .refine(data => Number.isInteger(+data), "Page must be integer!")
            .default("1"),
          limit: z
            .string()
            .min(1, "Invalid limit")
            .refine(data => Number.isInteger(+data), "Limit must be integer!")
            .default("20")
        })
      }
    },
    async ({ query }) => {
      const { limit, page } = query

      return PredictionEventRepository.findPaginate(+page, +limit)
    }
  )
}

export default predictionEventHandler
