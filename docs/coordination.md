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

### communication with PdM

- tasks and priorities are communicated directly by PdM
- ask PdM when clarification or decisions are needed
- report progress and blockers to PdM regularly
- propose task breakdowns for complex features

### work selection

1. choose tasks corresponding to your role
2. verify no other Claude Code instance is working on the same task
3. communicate with PdM when starting work
4. report completion to PdM

### avoiding duplicate work

- announce your work to PdM before starting
- check with PdM if unsure whether a task is already assigned
- communicate regularly about progress

## Git Workflow

### branching strategy

```
main (protected branch)
  ├── feature/backend-*   (Claude Code A)
  ├── feature/frontend-*  (Claude Code B)
  └── feature/infra-*     (Claude Code C)
```

### branch naming conventions

- backend: `feature/backend-<task-description>`
- frontend: `feature/frontend-<task-description>`
- infrastructure: `feature/infra-<task-description>`
- bug fixes: `fix/<component>-<bug-description>`

### work flow

1. create new feature branch from main
2. implement changes
3. commit (when instructed by user)
4. create PR
5. merge after review

### commit rules

- commit messages start with lowercase English
- never add Co-authored-by
- check changes with `git diff --cached` before committing
- report commit message to user

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

1. **always start work from latest main branch**
2. **commit small and frequently** (when user instructs)
3. **clear commit messages**
4. **write tests**
5. **update documentation**
6. **ask when unclear**
7. **consider impact of changes**
8. **communicate with PdM regularly**
