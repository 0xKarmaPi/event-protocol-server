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
    async ({ params, user }, reply) => {
      const id = +params.id
      const predictionEvent = await PredictionEventRepository.findByAuthorAndId(
        user.id,
        id
      )

      if (!predictionEvent) throw reply.notFound("Not found prediction event!")
      return PredictionEventRepository.delete(user.id, id)
    }
  )
}

export default predictionEventHandler
