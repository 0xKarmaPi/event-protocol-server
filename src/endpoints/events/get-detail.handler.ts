import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import { EventRepository } from "@root/repositories/event.repository.js"

const EventHandler: FastifyPluginAsyncZod = async self => {
  self.get(
    "/:id",
    {
      schema: {
        tags: ["Event"],
        params: z.object({
          id: z.string()
        })
      }
    },
    async ({ params }, reply) => {
      const Event = await EventRepository.findDetailById(+params.id)
      if (!Event) {
        return reply.notFound("Not found event!")
      }
      return Event
    }
  )
}

export default EventHandler
