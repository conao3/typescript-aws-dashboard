# Current Tasks

this document tracks current tasks for the typescript-aws-dashboard project.

## Task Status Legend

- 🔴 **TODO**: not started
- 🟡 **IN PROGRESS**: currently being worked on
- 🟢 **DONE**: completed
- ⚪ **BLOCKED**: waiting for dependencies

## Backend Tasks

### 🔴 implement health check endpoint

- **assigned to**: backend lead
- **priority**: low
- **description**: add health check endpoint for monitoring
- **dependencies**: none
- **notes**:

## Frontend Tasks

### 🔴 implement authentication UI

- **assigned to**: frontend lead
- **priority**: high
- **description**: create login/logout pages and authentication flow with React Aria Components
- **dependencies**: backend JWT authentication
- **notes**:

### 🔴 implement token management

- **assigned to**: frontend lead
- **priority**: high
- **description**: store and manage JWT tokens in cookies or localStorage
- **dependencies**: authentication UI
- **notes**:

### 🔴 setup Apollo Client with authentication

- **assigned to**: frontend lead
- **priority**: high
- **description**: configure Apollo Client to include JWT token in requests
- **dependencies**: token management
- **notes**:

### 🔴 create basic layout components

- **assigned to**: frontend lead
- **priority**: medium
- **description**: implement header, sidebar, and main layout components with React Aria Components
- **dependencies**: none
- **notes**:

### 🔴 implement AWS credentials UI

- **assigned to**: frontend lead
- **priority**: high
- **description**: create UI for managing AWS credentials (add, edit, delete)
- **dependencies**: authentication UI
- **notes**: never display secret keys after initial creation

### 🔴 implement EC2 AMI import list view

- **assigned to**: frontend lead
- **priority**: high
- **description**: create list view for EC2 AMI import tasks with filtering and search
- **dependencies**: Apollo Client setup
- **notes**: use React Aria Components Table

### 🔴 implement EC2 AMI import detail view

- **assigned to**: frontend lead
- **priority**: high
- **description**: create detail view showing full import task information
- **dependencies**: EC2 AMI import list view
- **notes**: display all AWS API response fields

### 🔴 implement tenant context display

- **assigned to**: frontend lead
- **priority**: low
- **description**: show current tenant information in UI
- **dependencies**: Apollo Client setup
- **notes**:

## DevOps Tasks

## Completed Tasks

### 🟢 add Docker configuration (2026-02-08)

- **assigned to**: devops lead
- **priority**: low
- **description**: create Dockerfile and docker-compose.yml for deployment
- **implementation**:
  - created `backend/Dockerfile` with multi-stage build (Rust 1.83 builder + Debian slim runtime)
  - created `frontend/Dockerfile` with multi-stage build (Node.js 22 builder + slim runtime)
  - created `docker-compose.prod.yml` for production deployment with postgres, backend, and frontend services
  - created `.dockerignore` files for backend and frontend to optimize build context
  - created `.env.example` with required environment variables template
  - added deployment section to README.md with setup instructions
  - added docker commands to Makefile: docker-build, docker-up, docker-down, docker-logs
  - added `.env` to .gitignore to prevent committing secrets
  - improved backend Dockerfile: added postgresql-client and psqldef for automated schema migration
  - created `backend/docker-entrypoint.sh` to automate database schema application on container startup
  - backend container now automatically waits for postgres and applies schema before starting server
  - updated README.md to reflect automated schema migration (no manual steps needed)

### 🟢 integrate AWS SDK for EC2 (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: integrate AWS SDK for Rust, implement EC2 client with tenant credentials
- **implementation**:
  - added AWS SDK dependencies: aws-config 1.1, aws-sdk-ec2 1.109, aws-credential-types 1.1
  - created `backend/src/aws.rs` with AwsClientFactory
  - implemented create_ec2_client method to build EC2 client from encrypted credentials
  - decrypts AWS access keys using CryptoConfig before creating client
  - supports custom region configuration per credential
  - integrated AwsClientFactory into GraphQL schema for use in resolvers
  - added Clone trait to CryptoConfig to support factory pattern

### 🟢 implement EC2 AMI import task sync (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: implement sync functionality to fetch AMI import tasks from AWS API
- **implementation**:
  - added fetch_import_image_tasks method to AwsClientFactory in `backend/src/aws.rs`
  - calls AWS EC2 DescribeImportImageTasks API using tenant credentials
  - added Ec2AmiImportTask model to `backend/src/models.rs`
  - implemented syncImportImageTasks GraphQL mutation in `backend/src/main.rs`
  - mutation fetches tasks from AWS and stores them in database with upsert logic
  - converts AWS SDK types to JSON for snapshot_details and tags fields
  - implements tenant isolation: only syncs tasks for authenticated tenant
  - uses ON CONFLICT to update existing tasks or insert new ones
  - returns list of synced tasks with all details
  - updated docs/api.md with Ec2AmiImportTask type and syncImportImageTasks mutation

### 🟢 implement EC2 GraphQL schema (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: define GraphQL schema for EC2 AMI import tasks queries and mutations
- **implementation**:
  - added Ec2AmiImportTaskFilter input type to `backend/src/models.rs`
  - filter supports aws_credential_id, status, import_task_id, limit, and offset
  - implemented ec2AmiImportTasks query in `backend/src/main.rs` with filtering and pagination
  - dynamically builds SQL query based on filter parameters
  - default limit is 100, max limit is 1000
  - implemented ec2AmiImportTask query to fetch single task by ID
  - enforces tenant isolation on all queries
  - updated docs/api.md with ec2AmiImportTasks and ec2AmiImportTask queries
  - added Ec2AmiImportTaskFilter type documentation
  - added usage examples with and without filters

### 🟢 implement AWS credentials management (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: implement CRUD operations for AWS credentials with encryption
- **implementation**:
  - created `backend/src/crypto.rs` with AES-256-GCM encryption/decryption
  - added dependencies: aes-gcm 0.10, base64 0.22, rand 0.8, hex 0.4
  - implemented CryptoConfig with encrypt/decrypt methods using ENCRYPTION_KEY environment variable
  - created AwsCredential and AwsCredentialRow models in `backend/src/models.rs`
  - added Input types: CreateAwsCredentialInput, UpdateAwsCredentialInput
  - implemented awsCredentials and awsCredential queries for listing and retrieving credentials
  - implemented createAwsCredential mutation with automatic encryption of access keys
  - implemented updateAwsCredential mutation for name and region updates (keys cannot be updated)
  - implemented deleteAwsCredential mutation with tenant isolation
  - credentials are encrypted before storage and never exposed in API responses
  - updated docs/api.md with complete AWS credentials schema and usage examples

### 🟢 setup initial GraphQL schema (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: define basic GraphQL schema with tenant-aware queries and mutations
- **implementation**:
  - added registerTenant mutation to create new tenants with admin user
  - added createUser mutation for tenant-aware user creation
  - added logout mutation (returns true, token invalidation handled client-side)
  - exposed Tenant model as SimpleObject in GraphQL schema
  - added Input types: RegisterTenantInput, CreateUserInput
  - implemented transaction for tenant + admin user creation
  - updated docs/api.md with complete GraphQL schema documentation
  - added usage examples for all mutations and queries
  - documented authentication flow with JWT token in Authorization header

### 🟢 setup CI/CD pipeline (2026-02-08)

- **assigned to**: devops lead
- **priority**: medium
- **description**: configure GitHub Actions for testing and building
- **implementation**:
  - created `.github/workflows/ci.yml` with GitHub Actions workflow
  - configured Nix environment setup with cachix/install-nix-action@v30
  - added DeterminateSystems/magic-nix-cache-action for faster builds
  - workflow runs on push to main/master and pull requests
  - steps include: nix flake check, dependency installation, formatting check, lint, build, and tests
  - added PostgreSQL 17 service container for future database tests
  - added `lint` script to frontend/package.json (tsc --noEmit)
  - CI runs in Nix development environment for consistency with local development

### 🟢 implement tenant context middleware (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: create middleware to extract tenant_id from JWT and inject into request context
- **implementation**:
  - created `backend/src/middleware.rs` with TenantContext struct
  - implemented extract_tenant_context to parse Authorization header and verify JWT
  - integrated middleware into graphql handler in `backend/src/main.rs`
  - added TenantContext injection into GraphQL execution context
  - created currentUser query to demonstrate tenant-aware GraphQL queries
  - added async-graphql uuid and chrono features to support User model in GraphQL
  - User model marked as SimpleObject with password_hash field excluded from GraphQL schema

### 🟢 setup PostgreSQL database connection (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: configure PostgreSQL connection pool and basic database setup
- **implementation**:
  - added sqlx 0.8 with PostgreSQL, UUID, and chrono support
  - created `backend/src/db.rs` with connection pool initialization
  - integrated pool into actix-web application state
  - added `/health` endpoint for database connectivity check
  - pool configuration: max 5 connections, DATABASE_URL environment variable support

### 🟢 setup sqldef for schema management (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: install psqldef and create initial schema.sql file structure
- **implementation**:
  - added sqldef and postgresql to flake.nix development environment
  - created `backend/schema.sql` with dashboard schema and tenants table
  - added `make db-migrate-dry` for schema change preview
  - added `make db-migrate` for schema application
  - successfully applied initial schema with tenants table, indexes, and constraints

### 🟢 create initial schema with multi-tenancy (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: create backend/schema.sql with tenants table and initial schema with tenant_id columns
- **implementation**:
  - created `dashboard.users` table with tenant_id, authentication fields
  - created `dashboard.aws_credentials` table with encrypted credential storage
  - created `dashboard.ec2_ami_import_tasks` table for AMI import tracking
  - all tables include tenant_id with foreign key to tenants(id) on delete cascade
  - added tenant_id indexes on all tenant-specific tables
  - added unique constraints: users(tenant_id, email), ec2_ami_import_tasks(tenant_id, import_task_id)

### 🟢 implement JWT authentication (2026-02-08)

- **assigned to**: backend lead
- **priority**: high
- **description**: implement JWT token generation, validation, and tenant_id extraction
- **implementation**:
  - added jsonwebtoken 9 and bcrypt 0.16 dependencies
  - created `backend/src/auth.rs` with JwtConfig, Claims, and password hashing utilities
  - created `backend/src/models.rs` with User and Tenant database models
  - implemented `login` GraphQL mutation with email/password authentication
  - JWT includes user_id (sub), tenant_id, email, and expiration (24 hours)
  - created `backend/seed.sql` with test tenants and users (password: "password")
  - login mutation validates credentials and returns JWT token

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
