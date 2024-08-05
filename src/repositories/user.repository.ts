import type { Prisma } from "@prisma/client"

import { prisma } from "@root/infrastrutures/database.js"

export abstract class UserRepository {
  static async create(data: Prisma.UserCreateInput) {
    return prisma.user.create({ data })
  }

  static async findById(id: number) {
    return prisma.user.findUnique({ where: { id } })
  }

  static async findByAddress(address: string) {
    return prisma.user.findUnique({ where: { address } })
  }
}
