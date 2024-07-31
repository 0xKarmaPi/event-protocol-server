import * as crypto from "node:crypto"
import * as fs from "node:fs"

export const decryptEnv = (secret: string) => {
  const ivLength = 16

  const fileBuffer = fs.readFileSync(".env.enc")

  const iv = fileBuffer.subarray(0, ivLength)

  const cipher = fileBuffer.subarray(ivLength, fileBuffer.length)

  const key = crypto.createHash("sha256").update(secret).digest()

  const decipher = crypto.createDecipheriv("aes256", key, iv)

  let decrypted = decipher.update(cipher, undefined, "utf8")

  decrypted += decipher.final("utf8")

  return decrypted
}
