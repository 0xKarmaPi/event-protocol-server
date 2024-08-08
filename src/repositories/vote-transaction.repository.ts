import { prisma } from "@root/infrastrutures/database.js"

type CreateVoteTransaction = {
  userId: number
  answerId: number
  amount: number
  signedContractId: string
}

export abstract class VoteTransactionRepository {
  static async create(data: CreateVoteTransaction) {
    return prisma.voteTransaction.create({
      data
    })
  }
}
