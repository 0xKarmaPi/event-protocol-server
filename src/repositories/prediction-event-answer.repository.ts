import type { Prisma } from "@prisma/client"

import { prisma } from "@root/infrastrutures/database.js"

export abstract class PredictionAnswerRepository {
  static async findById(id: number) {
    return prisma.predictionAnswer.findUnique({ where: { id } })
  }

  static async create(
    predictionEventId: number,
    data: Prisma.PredictionAnswerCreateWithoutPredictionEventInput
  ) {
    return prisma.predictionAnswer.create({
      data: {
        ...data,
        predictionEventId
      }
    })
  }

  static async deleteById(id: number) {
    return prisma.predictionAnswer.delete({ where: { id } })
  }
}
