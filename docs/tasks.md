# Current Tasks

this document tracks current tasks for the typescript-aws-dashboard project.

## Task Status Legend

- 🔴 **TODO**: not started
- 🟡 **IN PROGRESS**: currently being worked on
- 🟢 **DONE**: completed
- ⚪ **BLOCKED**: waiting for dependencies

## Backend Tasks

### 🔴 setup initial GraphQL schema
- **assigned to**: backend lead
- **priority**: high
- **description**: define basic GraphQL schema structure with Query and Mutation types
- **dependencies**: none
- **notes**:

### 🔴 implement health check endpoint
- **assigned to**: backend lead
- **priority**: medium
- **description**: add health check endpoint for monitoring
- **dependencies**: none
- **notes**:

## Frontend Tasks

### 🔴 setup GraphQL client
- **assigned to**: frontend lead
- **priority**: high
- **description**: configure GraphQL client to connect to backend
- **dependencies**: backend GraphQL schema must be defined
- **notes**:

### 🔴 create basic layout components
- **assigned to**: frontend lead
- **priority**: medium
- **description**: implement header, sidebar, and main layout components
- **dependencies**: none
- **notes**:

## DevOps Tasks

### 🔴 setup CI/CD pipeline
- **assigned to**: devops lead
- **priority**: medium
- **description**: configure GitHub Actions for testing and building
- **dependencies**: none
- **notes**:

### 🔴 add Docker configuration
- **assigned to**: devops lead
- **priority**: low
- **description**: create Dockerfile and docker-compose.yml for deployment
- **dependencies**: none
- **notes**:

## Completed Tasks

tasks marked as 🟢 DONE will be moved here with completion date.

---

## How to Use This File

### adding a new task

copy this template:

```markdown
### 🔴 task title
- **assigned to**: role (backend lead / frontend lead / devops lead)
- **priority**: high / medium / low
- **description**: detailed description
- **dependencies**: list any blocking tasks or "none"
- **notes**: additional information
```

### updating task status

change the emoji at the start of the task:
- starting work: 🔴 → 🟡
- completing work: 🟡 → 🟢
- blocked: any → ⚪

### completing a task

when a task is done:
1. change status to 🟢
2. add completion date to notes
3. move to "Completed Tasks" section

### communication

- announce when starting a task (🔴 → 🟡)
- update notes with progress or blockers
- notify PdM when completing a task (🟡 → 🟢)
