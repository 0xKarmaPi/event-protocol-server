/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable importHelpers/order-imports */
import authPlugin from "@root/plugins/auth.plugin.js"
import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

const handler: FastifyPluginAsyncZod = async self => {
  self.register(authPlugin).post(
    "/sign",
    {
      schema: {
        tags: ["Auth"],
        body: z.object({
          id: z.string()
        })
      }
    },
    async ({ body, user }) => {
      return {
        msg: "ok"
      }
    }
  )
}

export default handler
