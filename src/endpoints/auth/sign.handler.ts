import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"

const handler: FastifyPluginAsyncZod = async self => {
  self.post(
    "/sign",
    {
      schema: {
        tags: ["Auth"]
      }
    },
    async () => {
      return {
        msg: "ok"
      }
    }
  )
}

export default handler
