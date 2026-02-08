# Documentation

This directory contains documentation for the typescript-aws-dashboard project.

## Document List

### Project Management

- **[coordination.md](./coordination.md)** - guidelines for multiple Claude Code instances to work together
  - role assignments (backend/frontend/devops)
  - git workflow
  - information sharing methods
  - best practices

- **[task-management.md](./task-management.md)** - task management using vibe_kanban
  - vibe_kanban basic operations
  - task lifecycle
  - workflow
  - best practices

### Technical Documentation

- **[architecture.md](./architecture.md)** - overall system architecture
  - system overview
  - backend structure
  - frontend structure
  - deployment

- **[development.md](./development.md)** - development environment and workflow
  - setup instructions
  - development flow
  - coding conventions
  - troubleshooting

- **[api.md](./api.md)** - GraphQL API specification (to be created)
  - schema definitions
  - queries and mutations
  - type definitions
  - usage examples

## How to Read the Documentation

### for new project members

1. [architecture.md](./architecture.md) - understand the overall system
2. [development.md](./development.md) - set up development environment
3. [coordination.md](./coordination.md) - learn collaboration rules
4. [task-management.md](./task-management.md) - learn how to manage tasks

### for Claude Code instances starting work

1. [coordination.md](./coordination.md) - check your role
2. [task-management.md](./task-management.md) - select a task
3. [development.md](./development.md) - follow development procedures
4. [api.md](./api.md) - reference API specification (as needed)

### for PdM (product manager)

1. [task-management.md](./task-management.md) - task management methods
2. [coordination.md](./coordination.md) - team-wide rules
3. [architecture.md](./architecture.md) - understand technical constraints

## Updating Documentation

documentation must be kept up to date.

### when to update

- when adding new features
- when changing architecture
- when improving workflow
- when discovering new best practices

### update responsibility

- **coordination.md**: devops lead or PdM
- **task-management.md**: devops lead or PdM
- **architecture.md**: component owner
- **development.md**: devops lead
- **api.md**: backend lead

## Feedback

if you have suggestions for improvement or questions about the documentation, report to the PdM.
