import type { Prisma } from "@prisma/client"

import { prisma } from "@root/infrastrutures/database.js"

export abstract class EventRepository {
  static async findAll() {
    return prisma.event.findMany({
      include: {
        options: {
          select: {
            id: true,
            address: true,
            token: true,
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

    const total = await prisma.event.count()
    const data = await prisma.event.findMany({
      skip,
      take: limit,
      orderBy: {
        createdAt: "desc"
      },
      include: {
        options: {
          select: {
            id: true,
            address: true,
            token: true,
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
    return prisma.event.findUnique({
      where: { id, userId },
      include: {
        options: {
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
    return prisma.event.findUnique({
      where: { id },
      include: {
        options: {
          select: {
            id: true,
            address: true,
            token: true,
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
    return prisma.event.findUnique({
      where: { id },
      include: {
        options: {
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
    data: Prisma.EventCreateWithoutAuthorInput
  ) {
    return prisma.event.create({
      data: {
        ...data,
        userId
      }
    })
  }

  static async delete(userId: number, id: number) {
    return prisma.event.delete({
      where: {
        id,
        userId
      }
    })
  }
}
