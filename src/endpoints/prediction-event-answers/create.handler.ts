import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import { PredictionAnswerRepository } from "@root/repositories/prediction-event-answer.repository.js"
import { PredictionEventRepository } from "@root/repositories/prediction-event.repository.js"

const predictionEventHandler: FastifyPluginAsyncZod = async self => {
  self.post(
    "/",
    {
      schema: {
        tags: ["Prediction Answer"],
        body: z.object({
          predictionEventId: z.number(),
          value: z.string().max(500, "Title is maximum 500 characters").trim(),
          acceptableVoteTick: z
            .string()
            .max(100, "Acceptable vote tick is maximum 100 characters")
            .trim(),
          isCorrect: z.boolean().optional()
        })
      }
    },
    async ({ body }, reply) => {
      const { predictionEventId, ...payload } = body
      // Check exist prediction event
      const predictionEvent =
        await PredictionEventRepository.findDetailById(predictionEventId)
      if (!predictionEvent)
        return reply
          .status(406)
          .send({ message: "Not found prediction event!" })

      // Check prediction event over 2 answers
      if (predictionEvent.answers.length > 2) {
        return reply.status(406).send({ message: "Over two answers!" })
      }

      // Check duplicate correct or incorrect answer: isCorrect, value
      const isDuplicatedAnswer = predictionEvent.answers.some(
        answer =>
          answer.isCorrect === payload.isCorrect ||
          answer.value === payload.value
      )

      if (isDuplicatedAnswer) {
        return reply.status(406).send({
          message: `Duplicated ${payload.isCorrect ? "correct" : "incorrect"} answer!`
        })
      }

      return PredictionAnswerRepository.create(predictionEventId, payload)
    }
  )
}

export default predictionEventHandler
