import type { FastifyPluginAsync } from "fastify"
import fastifyPlugin from "fastify-plugin"

const authPlugin: FastifyPluginAsync = async self => {
  self.addHook("onRequest", async (request, reply) => {
    try {
      await request.jwtVerify()
    } catch (err) {
      throw reply.unauthorized()
    }
  })
}

export default fastifyPlugin(authPlugin)
