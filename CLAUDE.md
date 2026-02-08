# Project Instructions for Claude Code

this file contains project-specific instructions for Claude Code instances working on this project.

## Documentation

before starting any work, read the project documentation in [docs/README.md](./docs/README.md).

key documents:

- [docs/coordination.md](./docs/coordination.md) - coordination guidelines for multiple Claude Code instances
- [docs/tasks.md](./docs/tasks.md) - current task tracking and assignments
- [docs/architecture.md](./docs/architecture.md) - system architecture and multi-tenancy design
- [docs/multi-tenancy.md](./docs/multi-tenancy.md) - multi-tenancy implementation guide
- [docs/database.md](./docs/database.md) - database setup and sqldef schema management
- [docs/development.md](./docs/development.md) - development workflow
- [docs/api.md](./docs/api.md) - GraphQL API specification

## Role-Based Work

this project uses role-based collaboration:

- **backend lead**: responsible for Rust backend code
- **frontend lead**: responsible for TypeScript frontend code
- **devops lead**: responsible for infrastructure and integration

check your assigned role in [docs/coordination.md](./docs/coordination.md) before starting work.

## Communication

all task assignments and coordination go through the PdM (product manager). always communicate with PdM about:

- task selection
- progress updates
- blockers
- major design decisions

## Development Server Restrictions

**IMPORTANT**: do NOT start backend or frontend development servers. the user is already running them in watch mode:

- backend server (port 17231) is already running
- frontend server (port 17232) is already running

Claude Code instances should never execute:

- `make dev-backend`
- `make dev`
- any command that starts these servers

the servers automatically reload when you make code changes.

## Getting Started

1. read [docs/README.md](./docs/README.md) for documentation overview
2. follow setup instructions in [docs/development.md](./docs/development.md)
3. check your role and responsibilities in [docs/coordination.md](./docs/coordination.md)
4. coordinate with PdM for task assignment
