import autoLoad from "@fastify/autoload"
import cors from "@fastify/cors"
import fastifySensible from "@fastify/sensible"
import type { SwaggerOptions } from "@fastify/swagger"
import fastifySwagger from "@fastify/swagger"
import fastifySwaggerUI from "@fastify/swagger-ui"
import fastify from "fastify"
import {
  jsonSchemaTransform,
  serializerCompiler,
  validatorCompiler
} from "fastify-type-provider-zod"
import { dirname, join } from "path"
import { fileURLToPath } from "url"

import { prisma } from "./infrastrutures/database.js"

declare module "fastify" {
  //   interface FastifyInstance {}
  //   interface FastifyRequest {}
}

function main() {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = dirname(__filename)

  const swaggerOption: SwaggerOptions = {
    openapi: {
      openapi: "3.1.0",
      info: {
        title: "Api documentation",
        version: "1.0.0"
      },
      components: {
        securitySchemes: {
          bearerAuth: {
            type: "http",
            scheme: "bearer",
            bearerFormat: "JWT"
          }
        }
      }
    },
    transform: jsonSchemaTransform
  }

  fastify()
    .setValidatorCompiler(validatorCompiler)
    .setSerializerCompiler(serializerCompiler)
    .setErrorHandler((error, request, reply) => {
      if (error.statusCode) return reply.send(error)
      console.error("endpoint", request.url, error)
      return reply.internalServerError()
    })
    .addHook("onReady", () =>
      prisma.$connect().then(() => {
        console.log("connected to db")
      })
    )
    .register(cors)
    .register(fastifySensible)
    .register(autoLoad, {
      dir: join(__dirname, "plugins"),
      matchFilter: path => path.startsWith("/_"),
      encapsulate: false
    })
    .register(fastifySwagger, swaggerOption)
    .register(fastifySwaggerUI, {
      routePrefix: "/docs"
    })
    .register(autoLoad, {
      dir: join(__dirname, "endpoints"),
      matchFilter: path => path.endsWith("handler.js")
    })
    .get("/", () => "hello world!")
    .listen({ port: 8080 }, (err, address) => {
      if (err) {
        console.error(err)
        process.exit(1)
      } else {
        console.log(`server is listening at ${address}`)
      }
    })
}

export default main
