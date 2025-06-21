import { join } from "path"
import { readdirSync, statSync } from "fs"
import type { BeltCommand, CommandContext } from "../lib/types.ts"
import { getGlobalOutput } from "../lib/output.ts"
import { formatFileSize, formatRelativeTime } from "../lib/utils.ts"

interface FileInfo {
  name: string
  path: string
  type: "file" | "directory" | "symlink"
  size: number
  modified: Date
  permissions: string
}

const list: BeltCommand = {
  name: "list",
  description: "List files and directories with various options",
  arguments: [
    {
      name: "path",
      description: "Path to list (default: current directory)",
      required: false
    }
  ],
  options: [
    {
      flags: "--sort <field>",
      description: "Sort by: name, size, modified, type",
      defaultValue: "name"
    },
    {
      flags: "--reverse",
      description: "Reverse sort order"
    },
    {
      flags: "--filter <pattern>",
      description: "Filter files by pattern (glob-style)"
    },
    {
      flags: "--type <type>",
      description: "Show only specific type: file, directory, symlink"
    },
    {
      flags: "--size",
      description: "Show file sizes"
    },
    {
      flags: "--date",
      description: "Show modification dates"
    }
  ],
  action: async (context: CommandContext) => {
    const output = getGlobalOutput()
    const { args, options } = context
    const commandOpts = context.command.opts()

    const targetPath = args[0] || process.cwd()
    const sortField = commandOpts.sort || "name"
    const reverse = commandOpts.reverse || false
    const filter = commandOpts.filter
    const typeFilter = commandOpts.type
    const showSize = commandOpts.size || false
    const showDate = commandOpts.date || false

    if (!["name", "size", "modified", "type"].includes(sortField)) {
      throw new Error(`Invalid sort field: ${sortField}`)
    }

    if (typeFilter && !["file", "directory", "symlink"].includes(typeFilter)) {
      throw new Error(`Invalid type filter: ${typeFilter}`)
    }

    if (options.verbose) {
      output.info(`Listing contents of: ${targetPath}`)
      if (options.recursive) output.info("Recursive mode enabled")
      if (filter) output.info(`Filtering by pattern: ${filter}`)
      if (typeFilter) output.info(`Showing only: ${typeFilter}`)
    }

    output.startSpinner({ text: "Scanning files..." })

    try {
      const files = await scanDirectory(targetPath, {
        recursive: options.recursive || false,
        showAll: options.all || false,
        filter,
        typeFilter
      })

      output.stopSpinner(true, `Found ${files.length} item${files.length === 1 ? "" : "s"}`)

      if (files.length === 0) {
        output.warn("No files found matching criteria")
        return
      }

      const sorted = sortFiles(files, sortField, reverse)

      if (options.pipe) {
        output.json({
          path: targetPath,
          count: sorted.length,
          files: sorted.map((f) => ({
            name: f.name,
            path: f.path,
            type: f.type,
            size: f.size,
            modified: f.modified.toISOString(),
            permissions: f.permissions
          }))
        })
        return
      }

      if (options.list) {
        for (const file of sorted) {
          output.log(file.path)
        }
        return
      }

      if (showSize || showDate || options.all) {
        const headers = ["Name", "Type"]
        if (showSize || options.all) headers.push("Size")
        if (showDate || options.all) headers.push("Modified")
        if (options.all) headers.push("Permissions")

        const tableData = sorted.map((file) => {
          const row: Record<string, string> = {
            Name: file.name,
            Type: file.type
          }

          if (showSize || options.all) {
            row.Size = file.type === "file" ? formatFileSize(file.size) : "-"
          }

          if (showDate || options.all) {
            row.Modified = formatRelativeTime(file.modified)
          }

          if (options.all) {
            row.Permissions = file.permissions
          }

          return row
        })

        output.table(tableData, headers)
      } else {
        const grouped = groupFilesByType(sorted)

        for (const [type, typeFiles] of Object.entries(grouped)) {
          if (typeFiles.length === 0) continue

          if (options.verbose) {
            output.info(`${type}s (${typeFiles.length}):`)
          }

          for (const file of typeFiles) {
            const icon = getFileIcon(file.type)
            const name = file.name
            output.log(`  ${icon} ${name}`)
          }

          if (options.verbose && Object.keys(grouped).length > 1) {
            output.br()
          }
        }
      }

      if (options.verbose) {
        const summary = generateSummary(sorted)
        output.br()
        output.box(summary, "Summary")
      }
    } catch (error) {
      output.stopSpinner(false, "Failed to scan directory")
      throw error
    }
  }
}

async function scanDirectory(
  path: string,
  options: {
    recursive: boolean
    showAll: boolean
    filter?: string
    typeFilter?: string
  }
): Promise<FileInfo[]> {
  const files: FileInfo[] = []

  try {
    const entries = readdirSync(path)

    for (const entry of entries) {
      if (!options.showAll && entry.startsWith(".")) {
        continue
      }

      const fullPath = join(path, entry)

      try {
        const stat = statSync(fullPath)
        const fileInfo: FileInfo = {
          name: entry,
          path: fullPath,
          type: stat.isDirectory() ? "directory" : stat.isSymbolicLink() ? "symlink" : "file",
          size: stat.size,
          modified: stat.mtime,
          permissions: stat.mode.toString(8).slice(-3)
        }

        if (options.filter && !entry.includes(options.filter)) {
          continue
        }

        if (options.typeFilter && fileInfo.type !== options.typeFilter) {
          continue
        }

        files.push(fileInfo)

        if (options.recursive && fileInfo.type === "directory") {
          try {
            const subFiles = await scanDirectory(fullPath, options)
            files.push(...subFiles)
          } catch {
            // Skip directories we can't read
          }
        }
      } catch {
        // Skip files we can't stat
      }
    }
  } catch (error) {
    throw new Error(`Cannot read directory: ${path}`)
  }

  return files
}

function sortFiles(files: FileInfo[], field: string, reverse: boolean): FileInfo[] {
  const sorted = [...files].sort((a, b) => {
    let comparison = 0

    switch (field) {
      case "name":
        comparison = a.name.localeCompare(b.name)
        break
      case "size":
        comparison = a.size - b.size
        break
      case "modified":
        comparison = a.modified.getTime() - b.modified.getTime()
        break
      case "type":
        comparison = a.type.localeCompare(b.type)
        break
    }

    return reverse ? -comparison : comparison
  })

  return sorted
}

function groupFilesByType(files: FileInfo[]): Record<string, FileInfo[]> {
  return files.reduce(
    (groups, file) => {
      const type = file.type
      if (!groups[type]) groups[type] = []
      groups[type].push(file)
      return groups
    },
    {} as Record<string, FileInfo[]>
  )
}

function getFileIcon(type: string): string {
  switch (type) {
    case "directory":
      return "📁"
    case "file":
      return "📄"
    case "symlink":
      return "🔗"
    default:
      return "❓"
  }
}

function generateSummary(files: FileInfo[]): string {
  const grouped = groupFilesByType(files)
  const totalSize = files.filter((f) => f.type === "file").reduce((sum, f) => sum + f.size, 0)

  const lines = [
    `Total items: ${files.length}`,
    `Directories: ${grouped.directory?.length || 0}`,
    `Files: ${grouped.file?.length || 0}`,
    `Symlinks: ${grouped.symlink?.length || 0}`,
    `Total size: ${formatFileSize(totalSize)}`
  ]

  return lines.join("\\n")
}

export default list
