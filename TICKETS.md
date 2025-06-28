# Jig CLI - Linear Ticket Plan & Hierarchy

## Overview

This document outlines the hierarchical ticket structure for implementing the jig CLI project in Linear. The plan follows the 9-phase implementation approach from the README, with tickets organized into epics, features, and tasks.

## Ticket Hierarchy Structure

```
🎯 Epic (Phase-level)
├── 🔧 Feature (Command group or major component)
│   └── ✅ Task (Individual command or specific implementation)
└── 📋 Task (Cross-cutting concerns)
```

## Phase 1: Core Infrastructure & Project Setup

**Epic: Core Infrastructure & Project Setup**

- **Feature: CLI Framework Setup**
    - Task: Set up main jig command structure with clap
    - Task: Implement global options (--version, --help, --yes, --json, --verbose, --quiet, --silent)
    - Task: Configure command abbreviation support
- **Feature: Configuration Management**
    - Task: Implement ~/.jig.yaml loading with saphyr/serde
    - Task: Implement secret file loading and merging
    - Task: Implement hierarchical secret resolution (key/file/env)
- **Feature: Core Utilities Module**
    - Task: Create shared utilities module structure
    - Task: Implement Git abstraction layer (CLI vs gix)
    - Task: Implement error handling and logging strategy
    - Task: Implement resolve_github_username utility

## Phase 2: Foundational Command Groups

**Epic: Foundational Command Groups**

- **Feature: jig crypto commands**
    - Task: Implement age encryption (encrypt command)
    - Task: Implement age decryption (decrypt command)
    - Task: Implement public key derivation
- **Feature: jig generate commands**
    - Task: Implement hex generation
    - Task: Implement password generation with entropy validation
    - Task: Implement xkcd password mode
    - Task: Implement age key generation
    - Task: Implement WireGuard key generation
    - Task: Implement JWT generation with claims
- **Feature: jig network commands**
    - Task: Implement DNS cache flush (OS-aware)
    - Task: Implement DNS lookup with custom servers
    - Task: Implement DNSSEC validation
- **Feature: jig tls commands**
    - Task: Implement TLS certificate retrieval
    - Task: Implement cipher suite listing

## Phase 3: Project & Git Management

**Epic: Project & Git Management**

- **Feature: Template System**
    - Task: Implement template listing
    - Task: Implement template repository updates
    - Task: Implement new template creation
    - Task: Implement Tera template processing
- **Feature: Project Commands**
    - Task: Implement project scaffolding from templates
    - Task: Implement project update with diff display
    - Task: Implement .jig.yaml tracking file
    - Task: Implement dependabot.yml generation
    - Task: Implement dependency version bumping
- **Feature: Git Integration**
    - Task: Implement simplified clone (username/repo)
    - Task: Implement latest commit hash retrieval
    - Task: Implement GitHub secrets management
    - Task: Implement batch repo update (yank)
- **Feature: Binary Release Management**
    - Task: Implement binary metadata storage
    - Task: Implement GitHub release downloading
    - Task: Implement binary installation and PATH management
    - Task: Implement binary updates and removal

## Phase 4: External API Integration

**Epic: External API Integration**

- **Feature: Dave.io API Client**
    - Task: Create base API client with authentication
    - Task: Implement ping endpoint
    - Task: Implement token management endpoints
- **Feature: Domain Commands**
    - Task: Integrate Domainr API for availability checks
    - Task: Implement RDAP client for expiry checks
    - Task: Implement nameserver lookup with fallback

## Phase 5: AI-Powered Features

**Epic: AI-Powered Features**

- **Feature: AI Utilities**
    - Task: Implement prepare_image_for_claude utility
    - Task: Implement ask_claude helper function
- **Feature: AI Commands**
    - Task: Implement AI image renaming
    - Task: Implement AI commit message generation
    - Task: Implement image alt text generation
    - Task: Implement image optimization API
    - Task: Implement ticket title/description generation

## Phase 6: Advanced Shell & System Integration

**Epic: Advanced Shell & System Integration**

- **Feature: Initialization System**
    - Task: Implement directory structure creation
    - Task: Implement template repository cloning
    - Task: Implement shell integration detection
    - Task: Create shell plugin installers
- **Feature: Workspace Management**
    - Task: Implement workspace listing
    - Task: Implement workspace switching logic
    - Task: Implement environment variable management
    - Task: Implement shell hooks (bash/zsh/fish)
- **Feature: Terminal Utilities**
    - Task: Implement system info visualization
    - Task: Implement XKCD comic display

## Phase 7: Polish & Easter Eggs

**Epic: Polish & Easter Eggs**

- **Task: Implement jig dance command**
- **Task: Integrate terminal effects libraries**
- **Task: Design animation sequences**
- **Task: Hide from help output**

## Phase 8: Future & Protocol Implementation

**Epic: Model Context Protocol**

- **Task: Design MCP server architecture**
- **Task: Implement stdio MCP server**
- **Task: Document protocol specification**
- **Task: Create integration examples**

## Phase 9: Testing, Documentation & Release

**Epic: Testing & Quality Assurance**

- **Feature: Test Suite**
    - Task: Write unit tests for crypto operations
    - Task: Write unit tests for generation utilities
    - Task: Write integration tests for CLI commands
    - Task: Implement test fixtures and mocks
- **Feature: CI/CD Pipeline**
    - Task: Configure GitHub Actions workflow
    - Task: Set up cross-platform builds
    - Task: Implement release automation
- **Feature: Documentation**
    - Task: Review and update README
    - Task: Ensure comprehensive --help messages
    - Task: Create installation guides
- **Feature: Distribution**
    - Task: Configure cargo package metadata
    - Task: Create Homebrew tap formula
    - Task: Test installation methods

## Cross-Cutting Concerns

**Epic: Cross-Cutting Infrastructure**

- **Task: Implement comprehensive error types**
- **Task: Create integration test framework**
- **Task: Set up benchmarking suite**
- **Task: Implement telemetry/analytics (optional)**
- **Task: Create developer documentation**

## Ticket Creation Strategy

### Parent-Child Relationships

1. Each Phase becomes an Epic
2. Each Feature becomes a parent ticket under its Epic
3. Each Task becomes a child ticket under its Feature
4. Cross-cutting tasks can be standalone or grouped

### Estimates

- Epics: No direct estimate (sum of children)
- Features: 5-13 points (depending on complexity)
- Tasks: 1-5 points each

### Priority Guidelines

- Phase 1-2: High priority (foundational)
- Phase 3-4: Medium priority (core features)
- Phase 5-6: Medium priority (advanced features)
- Phase 7-8: Low priority (nice-to-have)
- Phase 9: High priority (before release)

### Labels to Create

- `infrastructure`
- `cli-command`
- `api-integration`
- `ai-powered`
- `shell-integration`
- `testing`
- `documentation`
- `security`
- `performance`

## Implementation Notes

1. **Dependencies**: Tickets should be created with proper blocking relationships
2. **Assignee**: All tickets initially assigned to Dave Williams
3. **Project**: Consider creating a "Jig CLI v1.0" project to track progress
4. **Milestones**: Each phase could be a milestone
5. **Description**: Each ticket should include:
    - Acceptance criteria
    - Technical considerations
    - Links to relevant README sections
    - Dependencies on other tickets

## Total Ticket Count Estimate

- 9 Epics
- ~25 Features
- ~80-100 Tasks

This structure provides clear visibility into progress while keeping tickets at an actionable size.
