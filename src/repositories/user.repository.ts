import type { Prisma } from "@prisma/client"

import { prisma } from "@root/infrastrutures/database.js"

export abstract class UserRepository {
  static async findById(id: number) {
    return prisma.user.findUnique({ where: { id } })
  }

  static async findByAddress(address: string) {
    return prisma.user.findUnique({ where: { address } })
  }

  static async create(data: Prisma.UserCreateInput) {
    return prisma.user.create({ data })
  }

  static async updateById(id: number, data: Prisma.UserUpdateInput) {
    return prisma.user.update({
      where: {
        id
      },
      data
    })
  }
}
