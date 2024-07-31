import { prisma } from "@root/infrastrutures/database.js"

export abstract class UserRepository {
  static findByAddress(address: string) {
    return prisma.user.findUnique({
      where: {
        address
      }
    })
  }
}
