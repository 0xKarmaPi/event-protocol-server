export const safeParse = <T>(value: string) => {
  try {
    return JSON.parse(value) as T
  } catch (error) {
    return null
  }
}
