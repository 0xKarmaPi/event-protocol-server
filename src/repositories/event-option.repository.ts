import type { Prisma } from "@prisma/client"

import { prisma } from "@root/infrastrutures/database.js"

export abstract class EventOptionRepository {
  static async findById(id: number) {
    return prisma.eventOption.findUnique({ where: { id } })
  }

  static async create(
    EventId: number,
    data: Prisma.EventOptionCreateWithoutEventInput
  ) {
    return prisma.eventOption.create({
      data: {
        ...data,
        EventId
      }
    })
  }

  static async deleteById(id: number) {
    return prisma.eventOption.delete({ where: { id } })
  }
}
