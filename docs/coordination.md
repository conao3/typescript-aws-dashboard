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

### using vibe_kanban

project tasks are managed with vibe_kanban.

```bash
# list projects
list_projects

# list tasks
list_tasks project_id=<project-id>

# create task
create_task project_id=<project-id> title="..." description="..."

# update task
update_task task_id=<task-id> status=inprogress
```

### task statuses

- `todo`: not started
- `inprogress`: in progress
- `inreview`: awaiting review
- `done`: completed
- `cancelled`: cancelled

### how to select tasks

1. choose tasks corresponding to your role
2. choose tasks without dependencies or with resolved dependencies
3. always update status to `inprogress` when starting work
4. update status to `done` when completed

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
- check task status in vibe_kanban before starting work
- verify tasks when duplicate work is suspected

### interface contracts

boundary between backend and frontend:

- treat GraphQL API schema as a contract
- recognize that schema changes affect both sides
- update `api.md` when schema changes

## Communication

### coordination with PdM (product manager)

- ask PdM when clarification or decisions are needed
- PdM determines task priorities
- propose major design changes to PdM

### regular synchronization

- check progress of all Claude Code instances before important milestones
- pay special attention to interdependent tasks
- report blockers early

## Best Practices

1. **always start work from latest main branch**
2. **commit small and frequently** (when user instructs)
3. **clear commit messages**
4. **write tests**
5. **update documentation**
6. **ask when unclear**
7. **consider impact of changes**
