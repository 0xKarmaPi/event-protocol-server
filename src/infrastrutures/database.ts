import { PrismaClient } from "@prisma/client"

import { DATABASE_URL } from "@root/shared/env.js"

export const prisma = new PrismaClient({
  datasourceUrl: DATABASE_URL
})
