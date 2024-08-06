import type { Prisma } from "@prisma/client"

import { prisma } from "@root/infrastrutures/database.js"

export abstract class PredictionEventRepository {
  static async findAll() {
    return prisma.predictionEvent.findMany({
      include: {
        answers: {
          select: {
            id: true,
            balance: true,
            typeBalance: true,
            description: true
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

  static async findPaginate(page = 1, limit = 20) {
    const skip = (page - 1) * limit

    const total = await prisma.predictionEvent.count()
    const data = await prisma.predictionEvent.findMany({
      skip,
      take: limit,
      orderBy: {
        createdAt: "desc"
      },
      include: {
        answers: {
          select: {
            id: true,
            balance: true,
            typeBalance: true,
            description: true
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

    return {
      list: data,
      maxPage: Math.ceil(total / limit),
      total
    }
  }

  static async findByAuthorAndId(userId: number, id: number) {
    return prisma.predictionEvent.findUnique({
      where: { id, userId },
      include: {
        answers: {
          select: {
            id: true,
            isCorrect: true,
            description: true
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
            balance: true,
            typeBalance: true,
            description: true
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
            description: true
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
