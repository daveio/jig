# Template System Updates

This document outlines the changes needed to implement the shared component template system.

## 1. Directory Structure

We've created the following directory structure:

```
templates/
├── shared/             # Shared components
│   ├── gitignore/      # Common gitignore patterns
│   │   ├── common.gitignore     # Patterns common to all projects
│   │   ├── python.gitignore     # Python-specific patterns
│   │   └── rust.gitignore       # Rust-specific patterns
│   └── github/         # Shared GitHub workflows
│       └── workflows/  # GitHub Actions workflows
│           ├── base.yml.tera           # Base workflow template
│           ├── rust_jobs.yml.tera      # Rust-specific jobs
│           └── python_jobs.yml.tera    # Python-specific jobs
```

## 2. Placeholder Files

In each language directory, we use placeholder files to specify which shared components to use:

1. **Gitignore placeholders**:

   - A file named `gitignore` contains lines listing which gitignore files to include
   - Example: `common\npython` includes both common and python patterns

2. **Workflow placeholders**:
   - A file named `workflows` contains YAML defining workflow parameters
   - Example:
     ```yaml
     base:
       workflow_name: Python
       jobs_template: python_jobs
     ```

## 3. Code Changes Needed

### A. Template Module Updates

The `src/template/mod.rs` file needs to be updated to:

1. Detect placeholder files
2. Process them by combining shared components
3. Generate the final files

### B. Component Processor

A new component processor should:

1. Read placeholder files
2. Find the corresponding shared components
3. Process them based on the component type:
   - Gitignore: Concatenate files in order
   - Workflows: Use template variables to render the base template with the specified jobs

### C. Template Creation

The `process_template_dir` function should be updated to:

1. Detect placeholder files (no extension)
2. Process them differently from regular templates
3. Generate the combined file in the target location

## 4. Implementation Plan

1. Add a new `process_placeholder_file` function
2. Update the template walking code to detect placeholders
3. Add component-specific processors for each type (gitignore, workflow, etc.)
4. Update the documentation to explain the new system

## 5. Testing

Each component type should be tested:

1. Gitignore placeholders
2. Workflow placeholders
3. Any other shared component types

These changes maintain backward compatibility while enabling the new shared component system.
