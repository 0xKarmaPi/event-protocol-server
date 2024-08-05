import type { Prisma } from "@prisma/client"

import { prisma } from "@root/infrastrutures/database.js"

export abstract class PredictionEventRepository {
  static async findAll() {
    return prisma.predictionEvent.findMany({
      include: {
        answers: {
          select: {
            id: true,
            acceptableVoteTick: true,
            value: true
          }
        },
        author: {
          select: {
            username: true,
            address: true
          }
        }
      }
    })
  }

  static async findByAuthorAndId(userId: number, id: number) {
    return prisma.predictionEvent.findUnique({
      where: { id, userId },
      include: {
        answers: {
          select: {
            id: true,
            isCorrect: true,
            value: true
          }
        },
        author: {
          select: {
            username: true,
            address: true
          }
        }
      }
    })
  }

  static async findDetailById(id: number) {
    return prisma.predictionEvent.findUnique({
      where: { id },
      include: {
        answers: {
          select: {
            id: true,
            acceptableVoteTick: true,
            value: true
          }
        },
        author: {
          select: {
            username: true,
            address: true
          }
        }
      }
    })
  }

  static async findById(id: number) {
    return prisma.predictionEvent.findUnique({
      where: { id },
      include: {
        answers: {
          select: {
            id: true,
            isCorrect: true,
            value: true
          }
        },
        author: {
          select: {
            username: true,
            address: true
          }
        }
      }
    })
  }

  static async create(
    userId: number,
    data: Prisma.PredictionEventCreateWithoutAuthorInput
  ) {
    return prisma.predictionEvent.create({
      data: {
        ...data,
        userId
      }
    })
  }

  static async delete(userId: number, id: number) {
    return prisma.predictionEvent.delete({
      where: {
        id,
        userId
      }
    })
  }
}
