import type { Prisma } from "@prisma/client"

import { prisma } from "@root/infrastrutures/database.js"

export abstract class UserRepository {
  static async create(data: Prisma.UserCreateInput) {
    return prisma.user.create({ data })
  }

  static async findById(id: number) {
    return prisma.user.findUnique({ where: { id } })
  }

  static async findByEmail(email: string) {
    return prisma.user.findUnique({ where: { email } })
  }

  static async findByHashedCode(hashedCode: string) {
    return prisma.user.findUnique({ where: { hashedCode } })
  }

  static async findByEmailAndHashedCode({
    email,
    hashedCode
  }: {
    email: string
    hashedCode: string
  }) {
    return prisma.user.findUnique({ where: { email, hashedCode } })
  }
}
