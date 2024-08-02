import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

const handler: FastifyPluginAsyncZod = async self => {
  self.post(
    "/check-payload",
    {
      schema: {
        tags: ["Payload"],
        body: z.object({
          email: z.string().email(),
          payload: z.string()
        })
      }
    },
    async ({ query }) => {
      return query
    }
  )
}

export default handler
