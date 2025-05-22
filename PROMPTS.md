# Prompts

## Documentation Updates

```plaintext
Update the documentation for [...]

- `README.md`: for human consumption.
  - Include Mermaid diagrams where helpful.
- `CLAUDE.md`: for consumption by AI agents.
  - No diagrams.
- `SPEC.md`: for consumption by humans first but may be used by AI agents.
  - No diagrams.
  - Should allow an AI to recreate the project.
  - Should only talk about concepts, not any specifics of language, libraries, implementation.
  - Should be usable to recreate the project in any language with appropriate libraries.
  - Discussing logical flows on a high level is fine.
- `LEARNING.md`: for consumption by humans.
  - Uses the codebase to teach us about Rust and the libraries we use.
  - Can be as long as you like.
  - Include Mermaid diagrams where helpful.
  - Cover as much as possible.
  - Write in a friendly, slightly sardonic way.
```
