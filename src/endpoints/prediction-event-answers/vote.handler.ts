import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import authPlugin from "@root/plugins/auth.plugin.js"
import { PredictionAnswerRepository } from "@root/repositories/prediction-event-answer.repository.js"
import { VoteTransactionRepository } from "@root/repositories/vote-transaction.repository.js"
import { SECURITY } from "@root/shared/constant.js"

const voteAnswerHandler: FastifyPluginAsyncZod = async self => {
  self.register(authPlugin).post(
    "/:id/vote",
    {
      schema: {
        tags: ["Prediction Answer"],
        security: SECURITY,
        body: z.object({
          amount: z
            .number({ message: "Invalid amount" })
            .min(1, "Amount must be minimum 1!"),
          signedContractId: z.string()
        }),
        params: z.object({ id: z.string() })
      }
    },
    async ({ body, user, params }, reply) => {
      const answerId = +params.id

      // Check exist answer
      const answer = await PredictionAnswerRepository.findById(answerId)
      if (!answer) {
        throw reply.notFound("Not found answer")
      }

      return VoteTransactionRepository.create({
        ...body,
        answerId,
        userId: user.id
      })
    }
  )
}

export default voteAnswerHandler
