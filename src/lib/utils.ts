import { filesize } from "filesize"
import { existsSync, statSync } from "fs"
import { join } from "path"
import type { RuntimeInfo } from "./types.ts"

export function detectRuntime(): RuntimeInfo {
  const isBun = typeof Bun !== "undefined"
  const isNode = !isBun && typeof process !== "undefined"

  let version = "unknown"
  let platform = "unknown"

  if (isBun) {
    version = Bun.version
    platform = process?.platform || "unknown"
  } else if (isNode) {
    version = process.version
    platform = process.platform
  }

  return {
    isBun,
    isNode,
    version,
    platform
  }
}

export function ensureBunRuntime(): void {
  const runtime = detectRuntime()

  if (!runtime.isBun) {
    console.error("🥺 Oh no! This CLI was designed to run on Bun, not Node.js!")
    console.error("")
    console.error("🦄 Bun is like Node.js, but faster and more magical!")
    console.error("🚀 Install it with: curl -fsSL https://bun.sh/install | bash")
    console.error("📚 Or visit: https://bun.sh")
    console.error("")
    console.error("💡 Trust us, once you go Bun, you never go back! 🍞✨")
    process.exit(1)
  }
}

export function formatFileSize(bytes: number): string {
  return filesize(bytes, { standard: "jedec" })
}

export function formatRelativeTime(date: Date): string {
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const seconds = Math.floor(diff / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)
  const days = Math.floor(hours / 24)

  if (days > 0) return `${days} day${days > 1 ? "s" : ""} ago`
  if (hours > 0) return `${hours} hour${hours > 1 ? "s" : ""} ago`
  if (minutes > 0) return `${minutes} minute${minutes > 1 ? "s" : ""} ago`
  if (seconds > 10) return `${seconds} seconds ago`
  return "just now"
}

export async function fileExists(path: string): Promise<boolean> {
  try {
    await Bun.file(path).text()
    return true
  } catch {
    return false
  }
}

export async function isDirectory(path: string): Promise<boolean> {
  try {
    const stat = statSync(path)
    return stat.isDirectory()
  } catch {
    return false
  }
}

export async function isFile(path: string): Promise<boolean> {
  try {
    const stat = statSync(path)
    return stat.isFile()
  } catch {
    return false
  }
}

export async function readJsonFile<T = any>(path: string): Promise<T | null> {
  try {
    const content = await Bun.file(path).text()
    return JSON.parse(content) as T
  } catch {
    return null
  }
}

export async function writeJsonFile(path: string, data: any): Promise<boolean> {
  try {
    await Bun.write(path, JSON.stringify(data, null, 2))
    return true
  } catch {
    return false
  }
}

export function findProjectRoot(startPath: string = process.cwd()): string | null {
  let currentPath = startPath

  while (currentPath !== "/") {
    const packageJsonPath = join(currentPath, "package.json")
    if (existsSync(packageJsonPath)) {
      return currentPath
    }
    currentPath = join(currentPath, "..")
  }

  return null
}

export function validateEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email)
}

export function validateUrl(url: string): boolean {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

export function slugify(text: string): string {
  return text
    .toLowerCase()
    .replace(/[^a-z0-9\s-]/g, "")
    .replace(/\s+/g, "-")
    .replace(/-+/g, "-")
    .trim()
}

export function truncate(text: string, maxLength: number, suffix = "..."): string {
  if (text.length <= maxLength) return text
  return text.substring(0, maxLength - suffix.length) + suffix
}

export function parseKeyValuePairs(input: string): Record<string, string> {
  const pairs: Record<string, string> = {}

  for (const pair of input.split(",")) {
    const [key, value] = pair.split("=", 2)
    if (key && value) {
      pairs[key.trim()] = value.trim()
    }
  }

  return pairs
}

export function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export function randomId(length: number = 8): string {
  const chars = "abcdefghijklmnopqrstuvwxyz0123456789"
  let result = ""
  for (let i = 0; i < length; i++) {
    result += chars.charAt(Math.floor(Math.random() * chars.length))
  }
  return result
}

export function chunk<T>(array: T[], size: number): T[][] {
  const chunks: T[][] = []
  for (let i = 0; i < array.length; i += size) {
    chunks.push(array.slice(i, i + size))
  }
  return chunks
}

export function unique<T>(array: T[]): T[] {
  return [...new Set(array)]
}

export function groupBy<T>(array: T[], keyFn: (item: T) => string): Record<string, T[]> {
  return array.reduce(
    (groups, item) => {
      const key = keyFn(item)
      if (!groups[key]) groups[key] = []
      groups[key].push(item)
      return groups
    },
    {} as Record<string, T[]>
  )
}
