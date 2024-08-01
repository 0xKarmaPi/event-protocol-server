/* eslint-disable @typescript-eslint/no-unused-vars */
import type { FastifyPluginAsync } from "fastify"
import fastifyPlugin from "fastify-plugin"

const authPlugin: FastifyPluginAsync = async self => {
  self.addHook("onRequest", ({ headers }, reply) => {
    throw reply.unauthorized()
  })
}

export default fastifyPlugin(authPlugin)
