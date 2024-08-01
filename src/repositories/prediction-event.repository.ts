import type { Prisma } from "@prisma/client"

import { prisma } from "@root/infrastrutures/database.js"

export abstract class PredictionEventRepository {
  static async findAll() {
    return prisma.predictionEvent.findMany({
      include: {
        answers: {
          select: {
            id: true,
            isCorrect: true,
            acceptableVoteTick: true,
            value: true
          }
        }
      }
    })
  }

  static async findById(id: number) {
    return prisma.predictionEvent.findUnique({ where: { id } })
  }

  static async findDetailById(id: number) {
    return prisma.predictionEvent.findUnique({
      where: { id },
      include: {
        answers: {
          select: {
            id: true,
            isCorrect: true,
            acceptableVoteTick: true,
            value: true
          }
        }
      }
    })
  }

  static async create(data: Prisma.PredictionEventCreateInput) {
    return prisma.predictionEvent.create({ data })
  }

  static async deleteById(id: number) {
    return prisma.predictionEvent.delete({
      where: { id }
    })
  }
}
