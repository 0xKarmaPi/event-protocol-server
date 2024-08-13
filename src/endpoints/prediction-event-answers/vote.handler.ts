import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

import authPlugin from "@root/plugins/auth.plugin.js"
import { EventOptionRepository } from "@root/repositories/prediction-event-answer.repository.js"
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
      const optionId = +params.id

      // Check exist answer
      const answer = await EventOptionRepository.findById(optionId)
      if (!answer) {
        throw reply.notFound("Not found answer")
      }

      return VoteTransactionRepository.create({
        ...body,
        optionId,
        userId: user.id
      })
    }
  )
}

export default voteAnswerHandler
