# Claude Code Coordination Guide

this document provides guidelines for multiple Claude Code instances to collaborate on this project.

## Role Assignments

### Claude Code A: backend lead

- responsibility: `backend/` directory and Rust code
- main tasks:
  - GraphQL API implementation
  - business logic
  - database design
  - backend testing and refactoring

### Claude Code B: frontend lead

- responsibility: `frontend/` directory and TypeScript code
- main tasks:
  - UI/UX implementation
  - API client implementation
  - frontend testing and refactoring
  - responsive design

### Claude Code C: devops/integration lead

- responsibility: overall project integration and infrastructure
- main tasks:
  - CI/CD setup
  - overall testing strategy
  - build and deployment
  - documentation maintenance
  - dependency updates

## Task Management

tasks are tracked in [tasks.md](./tasks.md).

### communication with PdM

- tasks and priorities are communicated directly by PdM
- ask PdM when clarification or decisions are needed
- report progress and blockers to PdM regularly
- propose task breakdowns for complex features

### work selection

1. check [tasks.md](./tasks.md) for available tasks (🔴 TODO) in your role
2. verify no other Claude Code instance is working on the same task
3. update task status to 🟡 IN PROGRESS in [tasks.md](./tasks.md)
4. communicate with PdM when starting work
5. update task status to 🟢 DONE when completed
6. report completion to PdM

### avoiding duplicate work

- check [tasks.md](./tasks.md) for current task assignments (🟡 IN PROGRESS)
- update [tasks.md](./tasks.md) immediately when starting work
- announce your work to PdM before starting
- communicate regularly about progress

## Git Workflow

### branching strategy

work directly on the master branch in the main directory.

### commit rules

- commit messages start with lowercase English
- never add Co-authored-by
- check changes with `git diff --cached` before committing
- report commit message to user

### commit scope (CRITICAL)

**only commit changes within your responsibility area**:

- **backend lead**: only commit files in `backend/` directory
- **frontend lead**: only commit files in `frontend/` directory
- **devops lead**: only commit root-level config files (Makefile, docker-compose.yml, .github/, etc.)

**do NOT commit**:

- changes made by other Claude Code instances
- files outside your responsibility area
- documentation maintained by other roles

**documentation ownership**:

- coordination.md: devops lead or PdM
- tasks.md: devops lead or PdM (but all can update task status)
- architecture.md: component owner (backend/frontend lead for their sections)
- development.md: devops lead
- database.md: backend lead
- multi-tenancy.md: backend lead
- api.md: backend lead
- requirements.md: PdM only

when updating task status in tasks.md, only commit that specific change.

## Information Sharing

### using diary skill

each Claude Code should use the diary skill after work to record:

- completed work
- problems solved
- lessons learned (TIL)
- information to share with other Claude Code instances

### documentation updates

when there are important changes or new findings, update relevant documentation:

- `architecture.md`: architecture changes
- `development.md`: development procedure changes
- `api.md`: API specification changes

## Avoiding Conflicts

### avoiding file editing conflicts

- do not edit the same file simultaneously with multiple Claude Code instances
- communicate with PdM before starting work
- verify no duplicate work through PdM

### interface contracts

boundary between backend and frontend:

- treat GraphQL API schema as a contract
- recognize that schema changes affect both sides
- update `api.md` when schema changes
- notify both leads when schema changes

## Communication

### coordination with PdM (product manager)

- ask PdM when clarification or decisions are needed
- PdM determines task priorities
- propose major design changes to PdM
- report blockers early

### regular synchronization

- check progress with PdM before important milestones
- pay special attention to interdependent tasks
- communicate about dependencies between backend and frontend

## Best Practices

1. **always start work from latest master branch**
2. **commit small and frequently** (when user instructs)
3. **rebase and merge to master after completing work**
4. **clear commit messages**
5. **write tests**
6. **update documentation**
7. **ask when unclear**
8. **consider impact of changes**
9. **communicate with PdM regularly**
