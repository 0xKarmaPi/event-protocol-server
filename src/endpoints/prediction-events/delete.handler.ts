import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import authPlugin from "@root/plugins/auth.plugin.js"
import { PredictionEventRepository } from "@root/repositories/prediction-event.repository.js"
import { SECURITY } from "@root/shared/constant.js"

const predictionEventHandler: FastifyPluginAsyncZod = async self => {
  self.register(authPlugin).delete(
    "/:id",
    {
      schema: {
        tags: ["Prediction Event"],
        security: SECURITY,
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

      // return PredictionEventRepository.deleteById(id)
    }
  )
}

export default predictionEventHandler
