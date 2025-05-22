# Learning About Jig

_This document is written in the style of \_why the lucky stiff, the enigmatic Ruby programmer known for his whimsical and insightful writing in "Why's (Poignant) Guide to Ruby." \_why's style combines imaginative storytelling, quirky illustrations, and technical instruction in a way that makes learning both fun and memorable. If you enjoy this style, check out the original at `~/.poignant-guide`._

_[imagine a drawing of a fox wearing welding goggles, using a jig to perfectly align the pieces of a project]_

## CHAPTER 1: TEMPLATES AND TERA AWESOMENESS

So, we're diving into the absolutely fantabulous world of `jig` templates! These little snippets of code-joy are what make the `jig` tool dance and sing when creating new projects.

### The Template Directory Structure

Templates in `jig` live in the `templates/` directory, now with an ingenious shared component system:

```plaintext
templates/
├── shared/             # Shared components live here!
│   ├── gitignore/      # Common gitignore patterns
│   └── github/         # Shared GitHub workflows
├── rust/               # Rust-specific templates
├── python/             # Python-specific templates
├── javascript/         # JavaScript-specific templates
├── typescript/         # TypeScript-specific templates
├── go/                 # Go-specific templates
├── java/               # Java-specific templates
├── ruby/               # Ruby-specific templates
└── ... more languages coming soon!
```

Each language directory contains a complete project template with all the bells and whistles needed to kickstart development. But now, instead of duplicating common patterns, we have placeholder files that reference the shared components!

It's like having a bunch of foxes that all need the same tail. Instead of knitting a tail for each fox, we just make one perfect tail pattern and let each fox point to it. Magnificently efficient!

### Shared Components - The Code Reuse Revolution

In the `shared/` directory, we keep common components used by multiple language templates:

- **gitignore patterns**: Common patterns like `.DS_Store` and editor files live in `shared/gitignore/common.gitignore`
- **Language-specific gitignore patterns**: Patterns specific to each language live in `shared/gitignore/[language].gitignore`
- **GitHub workflow templates**: Base workflow structures live in `shared/github/workflows/`

Then, in each language directory, we have placeholder files that tell `jig` which shared components to use. For example, a `gitignore` file might simply contain:

```plaintext
common
python
```

This tells `jig` to include both the common gitignore patterns and the Python-specific ones. It's like a recipe that says "add 2 cups of common patterns and 1 cup of Python patterns" - and poof! You get a perfectly baked `.gitignore` file!

### Tera Templates - Where The Magic Happens

`jig` uses the Tera template engine (because it's TOTALLY AWESOME) to process files that need customization. Any file with a `.tera` extension will be processed by Tera during project creation.

Within these template files, variables are accessed using the `{{ variable_name }}` syntax. Here are the core variables available:

- `project_name`: The name of the project (as provided by the user)
- `language`: The programming language being used

For example, in a Python `__init__.py.tera` file:

```python
"""{{ project_name }} package."""

__version__ = "0.1.0"
```

When `jig` processes this file, it will replace `{{ project_name }}` with the actual project name, creating a nicely personalized module docstring.

### Fancy Transformations

Tera also supports filters, which let us transform variables in useful ways:

```tera
{{ project_name|lower|replace(from=" ", to="_") }}
```

This takes the project name, converts it to lowercase, and replaces spaces with underscores - perfect for creating valid Python module names!

Similarly, for creating package names in different formats:

```tera
// For kebab-case (used in package.json, etc.)
{{ project_name|lower|replace(from=" ", to="-") }}

// For PascalCase (used in Java/Ruby class names)
{{ project_name|capitalize|replace(from=" ", to="") }}
```

## CHAPTER 2: LANGUAGE-SPECIFIC AWESOMENESS

Each language template is crafted with love and consideration for that language's ecosystem and best practices.

### Python

Python templates use Poetry for dependency management, with a modern src-layout package structure. Testing is set up with pytest, and code quality is maintained with black, isort, mypy, and ruff.

### JavaScript/TypeScript

These templates include:

- Jest for testing
- ESLint for linting
- Prettier for formatting (TypeScript)
- TypeScript configuration (for TS projects)
- Modern Node.js practices

### Go

Go templates follow the standard Go project layout with:

- cmd/ directory for applications
- internal/ for private code
- pkg/ for public libraries
- Go modules configuration

### Java

Java templates use Maven for build management and include:

- JUnit 5 for testing
- Modern Java practices
- Maven plugins for building, testing, and packaging

### Ruby

Ruby templates are set up as gems with:

- RSpec for testing
- RuboCop for linting
- Modern Ruby practices
- Bundler for dependency management

## CHAPTER 3: WHEN TEMPLATES MEET THE REAL WORLD

When `jig` creates a new project, it takes these templates and works its magic:

1. Detects the requested language
2. Finds the appropriate template directory
3. Processes placeholder files to include shared components
4. Processes all `.tera` files, replacing variables with their values
5. Creates a new directory structure with all the files
6. Initializes a Git repository
7. Makes the first commit

The result? A shiny new project ready for development, with everything set up just right!

## CHAPTER 4: EXTENDING THE TEMPLATES

Want to add support for a new language or update an existing template? Here's how:

1. Check if there are shared components you can use in `templates/shared/`
2. Create a new directory under `templates/` for your language
3. Add placeholder files that reference shared components
4. Add language-specific files with `.tera` extensions where templating is needed
5. Add static files (no `.tera` extension) for files that don't need processing
6. Test your template with `jig new [your-language]`
7. Iterate until it's perfect!

Remember: a good template sets up not just the bare minimum, but includes:

- Testing frameworks
- Linting tools
- Build system configuration
- Documentation templates
- CI/CD configuration

If you find yourself creating patterns that might be useful for other languages, consider adding them to the `shared/` directory instead of duplicating them. Your future self (and other foxes) will thank you!

Happy templating, you marvelous code wizard! May your jigs always align perfectly and your projects compile on the first try!

_[imagine a drawing of the fox taking a bow, with perfectly aligned project pieces behind it]_
