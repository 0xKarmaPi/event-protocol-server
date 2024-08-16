import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import authPlugin from "@root/plugins/auth.plugin.js"
import { EventRepository } from "@root/repositories/event.repository.js"
import { SECURITY } from "@root/shared/constant.js"

const EventHandler: FastifyPluginAsyncZod = async self => {
  self.register(authPlugin).delete(
    "/:id",
    {
      schema: {
        tags: ["Event"],
        security: SECURITY,
        params: z.object({
          id: z.string()
        })
      }
    },
    async ({ params, user }, reply) => {
      const id = +params.id
      const event = await EventRepository.findByAuthorAndId(user.id, id)

      if (!event) throw reply.notFound("Not found event!")
      return EventRepository.delete(user.id, id)
    }
  )
}

export default EventHandler
