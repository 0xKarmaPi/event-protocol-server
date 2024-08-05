import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import authPlugin from "@root/plugins/auth.plugin.js"
import { PredictionAnswerRepository } from "@root/repositories/prediction-event-answer.repository.js"
import { PredictionEventRepository } from "@root/repositories/prediction-event.repository.js"
import { SECURITY } from "@root/shared/constant.js"

const predictionEventHandler: FastifyPluginAsyncZod = async self => {
  self.register(authPlugin).post(
    "/",
    {
      schema: {
        tags: ["Prediction Answer"],
        security: SECURITY,
        body: z.object({
          predictionEventId: z.number(),
          value: z.string().max(500, "Title is maximum 500 characters").trim(),
          acceptableVoteTick: z
            .string()
            .max(100, "Acceptable vote tick is maximum 100 characters")
            .toLowerCase()
            .trim(),
          isCorrect: z.boolean().optional()
        })
      }
    },
    async ({ body, user }, reply) => {
      const { predictionEventId, ...payload } = body

      const predictionEvent = await PredictionEventRepository.findByAuthorAndId(
        user.id,
        predictionEventId
      )
      if (!predictionEvent) throw reply.notFound("Not found prediction event!")

      // Check prediction event over 2 answers
      if (predictionEvent.answers.length === 2) {
        throw reply.notAcceptable("Event is over two answers!")
      }

      // Check duplicate correct or incorrect answer: isCorrect, value
      const isDuplicatedAnswer = predictionEvent.answers.some(
        answer =>
          answer.isCorrect === payload.isCorrect ||
          answer.value === payload.value
      )

      if (isDuplicatedAnswer) {
        throw reply.notAcceptable(
          `Duplicated value or ${payload.isCorrect ? "correct" : "incorrect"} answer!`
        )
      }

      return PredictionAnswerRepository.create(predictionEventId, payload)
    }
  )
}

export default predictionEventHandler
