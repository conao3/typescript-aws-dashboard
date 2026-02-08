# Project Requirements

this document defines the requirements and scope for typescript-aws-dashboard.

## Project Overview

typescript-aws-dashboard is a **multi-tenant SaaS application** that provides AWS management features missing from the official AWS Console.

### problem statement

the AWS Console lacks certain features and visibility into specific operations:

- **EC2 AMI import**: no way to view import status and details in the console
- other AWS service operations that are not visible or easily accessible
- limited customization and workflow automation

### solution

provide a dashboard that:

- fills gaps in AWS Console functionality
- organizes features by AWS service
- provides better visibility into AWS operations
- supports multiple organizations (multi-tenant)

## Target Users

- AWS users and organizations
- DevOps teams
- Cloud infrastructure managers
- companies managing AWS resources for multiple clients

## Core Features

### 1. multi-tenant architecture

- each organization (tenant) has isolated data
- users belong to a single tenant
- secure authentication with JWT
- tenant-specific AWS credentials management

### 2. AWS service modules

the application is organized by AWS service, with each service having its own module.

#### EC2 module (priority: high)

**AMI import tracking**:

- view all AMI import tasks
- monitor import progress
- view import task details (status, progress, errors)
- filter and search import tasks
- historical import task data

additional EC2 features (future):

- instance lifecycle management
- custom views and filters
- automation workflows

#### other AWS services (future)

modules will be added based on:

- AWS Console gaps
- user requests
- common pain points

examples:

- S3: advanced bucket analytics
- RDS: custom monitoring views
- Lambda: detailed execution analysis
- CloudFormation: stack dependency visualization

### 3. AWS credentials management

each tenant can:

- register AWS credentials (access key, secret key)
- manage multiple AWS accounts
- specify default region
- credentials stored securely (encrypted)

### 4. authentication and authorization

- JWT-based authentication
- role-based access control (RBAC)
- tenant isolation enforcement
- user management within tenant

### 5. dashboard and visualization

- overview dashboard per AWS service
- customizable widgets
- real-time data updates
- charts and graphs for metrics

## Technical Requirements

### backend

- Rust with actix-web
- GraphQL API with async-graphql
- PostgreSQL 17 for data storage
- sqldef for schema management
- AWS SDK for Rust for AWS API calls
- JWT for authentication
- tenant context middleware

### frontend

- TypeScript with TanStack Start
- React Aria Components for UI
- Apollo Client for GraphQL
- Tailwind CSS v4 for styling
- responsive design
- SSR support

### infrastructure

- PostgreSQL in Docker container
- Nix for development environment
- deployment-ready build

## Data Model

### core entities

```
tenant
  - id (uuid)
  - name (text)
  - slug (text, unique)
  - created_at (timestamptz)
  - is_active (boolean)

user
  - id (uuid)
  - tenant_id (uuid, fk)
  - email (text, unique per tenant)
  - name (text)
  - password_hash (text)
  - role (enum: admin, member)
  - created_at (timestamptz)

aws_credential
  - id (uuid)
  - tenant_id (uuid, fk)
  - name (text)
  - access_key_id (text, encrypted)
  - secret_access_key (text, encrypted)
  - default_region (text)
  - is_default (boolean)
  - created_at (timestamptz)

ec2_ami_import_task
  - id (uuid)
  - tenant_id (uuid, fk)
  - aws_credential_id (uuid, fk)
  - import_task_id (text) -- AWS import task ID
  - status (text)
  - progress (text)
  - status_message (text)
  - snapshot_id (text)
  - image_id (text)
  - description (text)
  - architecture (text)
  - platform (text)
  - created_at (timestamptz)
  - updated_at (timestamptz)
```

## API Structure

### GraphQL schema organization

organize by AWS service:

```graphql
type Query {
  # authentication
  currentUser: User

  # EC2
  ec2AmiImportTasks(filter: EC2AmiImportTaskFilter): [EC2AmiImportTask!]!
  ec2AmiImportTask(id: ID!): EC2AmiImportTask

  # future: other services
}

type Mutation {
  # authentication
  login(email: String!, password: String!): AuthPayload!
  logout: Boolean!

  # tenant/user management
  registerTenant(input: RegisterTenantInput!): Tenant!
  createUser(input: CreateUserInput!): User!

  # AWS credentials
  createAwsCredential(input: CreateAwsCredentialInput!): AwsCredential!
  updateAwsCredential(id: ID!, input: UpdateAwsCredentialInput!): AwsCredential!
  deleteAwsCredential(id: ID!): Boolean!

  # EC2
  refreshEc2AmiImportTasks: [EC2AmiImportTask!]!

  # future: other services
}
```

## Security Requirements

### authentication

- secure password hashing (bcrypt)
- JWT tokens with expiration
- token refresh mechanism
- secure credential storage (encryption at rest)

### authorization

- tenant isolation (all queries filtered by tenant_id)
- role-based access control
- AWS credential access control

### AWS credentials

- encrypt access keys and secrets in database
- never expose credentials in API responses
- use credentials only server-side
- support AWS IAM roles (future)

### data isolation

- all database queries must filter by tenant_id
- middleware enforcement
- no cross-tenant data access
- audit logging for sensitive operations

## Performance Requirements

- API response time: < 1 second for most queries
- AWS API calls: cached where appropriate
- real-time updates: WebSocket or polling
- pagination for large datasets

## Scalability

- support 100+ tenants initially
- horizontal scaling capability
- database optimization with proper indexes
- connection pooling

## Monitoring and Logging

- application logs
- error tracking
- AWS API call metrics
- tenant usage metrics
- performance monitoring

## Development Phases

### phase 1: foundation (current)

- [ ] multi-tenant authentication system
- [ ] database schema with tenant isolation
- [ ] AWS credentials management
- [ ] basic dashboard layout
- [ ] GraphQL API structure

### phase 2: EC2 AMI import (priority)

- [ ] AWS SDK integration for EC2
- [ ] AMI import task data model
- [ ] sync AMI import tasks from AWS
- [ ] display AMI import tasks in UI
- [ ] filter and search functionality
- [ ] detail view for import tasks

### phase 3: additional features

- [ ] other EC2 features
- [ ] additional AWS service modules
- [ ] advanced visualization
- [ ] automation workflows
- [ ] notifications and alerts

### phase 4: production readiness

- [ ] comprehensive testing
- [ ] CI/CD pipeline
- [ ] deployment automation
- [ ] documentation
- [ ] monitoring and alerting

## Success Criteria

- users can view EC2 AMI import tasks not visible in AWS Console
- multi-tenant isolation is secure and tested
- AWS credentials are securely stored and used
- application is responsive and performant
- clear path to add more AWS service modules

## Out of Scope (for now)

- AWS resource modification (read-only initially)
- billing and usage tracking
- cost optimization recommendations
- AWS Organizations integration
- custom scripting/automation engine

## Future Considerations

- AWS IAM role integration
- SSO integration
- mobile application
- API for third-party integrations
- white-label capability
- terraform/CloudFormation generation
