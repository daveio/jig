# Learning About Jig

_[imagine a drawing of a fox wearing welding goggles, using a jig to perfectly align the pieces of a project]_

## CHAPTER 1: TEMPLATES AND TERA AWESOMENESS

So, we're diving into the absolutely fantabulous world of `jig` templates! These little snippets of code-joy are what make the `jig` tool dance and sing when creating new projects.

### The Template Directory Structure

Templates in `jig` live in the `templates/` directory, neatly organized by language:

```
templates/
├── rust/
├── python/
├── javascript/
├── typescript/
├── go/
├── java/
├── ruby/
└── ... more languages coming soon!
```

Each language directory contains a complete project template with all the bells and whistles needed to kickstart development. These include:

- Package manager configuration files (Cargo.toml, pyproject.toml, package.json, etc.)
- Source code directory structure
- Sample code files
- Test files and directories
- GitHub Actions workflows
- README files
- .gitignore files
- And other language-specific goodies!

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

```
{{ project_name|lower|replace(from=" ", to="_") }}
```

This takes the project name, converts it to lowercase, and replaces spaces with underscores - perfect for creating valid Python module names!

Similarly, for creating package names in different formats:

```
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
3. Processes all `.tera` files, replacing variables with their values
4. Creates a new directory structure with all the files
5. Initializes a Git repository
6. Makes the first commit

The result? A shiny new project ready for development, with everything set up just right!

## CHAPTER 4: EXTENDING THE TEMPLATES

Want to add support for a new language or update an existing template? Here's how:

1. Create a new directory under `templates/` for your language
2. Add all the necessary files with `.tera` extensions where templating is needed
3. Add static files (no `.tera` extension) for files that don't need processing
4. Test your template with `jig new [your-language]`
5. Iterate until it's perfect!

Remember: a good template sets up not just the bare minimum, but includes:

- Testing frameworks
- Linting tools
- Build system configuration
- Documentation templates
- CI/CD configuration

Happy templating, you marvelous code wizard! May your jigs always align perfectly and your projects compile on the first try!

_[imagine a drawing of the fox taking a bow, with perfectly aligned project pieces behind it]_
