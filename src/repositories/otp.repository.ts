import type { Prisma } from "@prisma/client"

import { prisma } from "@root/infrastrutures/database.js"
export abstract class OtpRepository {
  static async create(data: Prisma.OtpCreateInput) {
    return prisma.otp.create({ data })
  }

  static async deleteById(id: number) {
    return prisma.otp.delete({ where: { id } })
  }
}
