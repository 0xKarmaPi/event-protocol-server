import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import authPlugin from "@root/plugins/auth.plugin.js"
import { EventRepository } from "@root/repositories/prediction-event.repository.js"
import { SECURITY } from "@root/shared/constant.js"

const EventHandler: FastifyPluginAsyncZod = async self => {
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
      const Event = await EventRepository.findByAuthorAndId(user.id, id)

      if (!Event) throw reply.notFound("Not found prediction event!")
      return EventRepository.delete(user.id, id)
    }
  )
}

export default EventHandler
