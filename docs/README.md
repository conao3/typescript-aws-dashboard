# Documentation

this directory contains documentation for the typescript-aws-dashboard project.

## Document List

### Project Management

- **[requirements.md](./requirements.md)** - project requirements and scope
  - project overview and goals
  - target users and features
  - technical requirements
  - development phases

- **[coordination.md](./coordination.md)** - guidelines for multiple Claude Code instances to work together
  - role assignments (backend/frontend/devops)
  - git workflow
  - information sharing methods
  - best practices

- **[tasks.md](./tasks.md)** - current task tracking
  - task status and assignments
  - priorities and dependencies
  - task management workflow

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

- **[multi-tenancy.md](./multi-tenancy.md)** - multi-tenancy implementation guide
  - design overview
  - database schema
  - authentication flow
  - security best practices

- **[database.md](./database.md)** - database setup and management
  - PostgreSQL configuration
  - sqldef schema management
  - migration workflow
  - common operations

## How to Read the Documentation

### for new project members

1. [architecture.md](./architecture.md) - understand the overall system
2. [development.md](./development.md) - set up development environment
3. [coordination.md](./coordination.md) - learn collaboration rules

### for Claude Code instances starting work

1. [coordination.md](./coordination.md) - check your role
2. [development.md](./development.md) - follow development procedures
3. [api.md](./api.md) - reference API specification (as needed)

### for PdM (product manager)

1. [coordination.md](./coordination.md) - team-wide rules
2. [architecture.md](./architecture.md) - understand technical constraints

## Updating Documentation

documentation must be kept up to date.

### when to update

- when adding new features
- when changing architecture
- when improving workflow
- when discovering new best practices

### update responsibility

- **requirements.md**: PdM only
- **coordination.md**: devops lead or PdM
- **tasks.md**: all (for task status updates), devops lead or PdM (for structure changes)
- **architecture.md**: component owner (backend/frontend lead for their sections)
- **development.md**: devops lead
- **database.md**: backend lead
- **multi-tenancy.md**: backend lead
- **api.md**: backend lead

## Feedback

if you have suggestions for improvement or questions about the documentation, report to the PdM.
