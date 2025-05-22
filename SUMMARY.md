# Template System Reorganization

_[imagine a fox juggling template files, with shared components glowing brightly in the center]_

## Changes Made

We've reorganized the template system to use shared components:

1. **Created Shared Component Directory**:

   - `templates/shared/gitignore/` for common gitignore patterns
   - `templates/shared/github/workflows/` for shared GitHub workflow templates

2. **Extracted Common Patterns**:

   - Common gitignore patterns (IDE files, logs, env vars) into `common.gitignore`
   - Language-specific gitignore patterns into dedicated files
   - Base GitHub workflow structure into `base.yml.tera`
   - Language-specific workflow jobs into dedicated files

3. **Created Placeholder System**:

   - Simple text files that reference shared components
   - No template language in filenames
   - Logic extracted to the files themselves

4. **Updated Documentation**:
   - README.md: Added section on the template sharing system
   - LEARNING.md: Updated to explain the new structure, in \_why's style
   - CLAUDE.md: Detailed explanation for AI assistants
   - SPEC-UPDATES.md: Implementation plan for the code changes

## Next Steps

To fully implement this system, the following code changes are needed:

1. Update `src/template/mod.rs` to process placeholder files
2. Add component-specific processors for each type
3. Update the template walking code
4. Write tests for the new functionality

These changes will maintain backward compatibility while enabling the new shared component system, making templates more maintainable and reducing duplication.
