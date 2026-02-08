# API Specification

this document defines the GraphQL API specification.

## Endpoints

- **development**: http://localhost:17231/graphql
- **GraphiQL IDE**: http://localhost:17231/admin/graphiql

## Schema

### Query

```graphql
type Query {
  hello: String!
  version: String!
  currentUser: User!
  awsCredentials: [AwsCredential!]!
  awsCredential(id: ID!): AwsCredential!
  ec2AmiImportTasks(filter: Ec2AmiImportTaskFilter): [Ec2AmiImportTask!]!
  ec2AmiImportTask(id: ID!): Ec2AmiImportTask!
}
```

### Mutation

```graphql
type Mutation {
  login(email: String!, password: String!): String!
  logout: Boolean!
  registerTenant(input: RegisterTenantInput!): Tenant!
  createUser(input: CreateUserInput!): User!
  createAwsCredential(input: CreateAwsCredentialInput!): AwsCredential!
  updateAwsCredential(id: ID!, input: UpdateAwsCredentialInput!): AwsCredential!
  deleteAwsCredential(id: ID!): Boolean!
  syncImportImageTasks(awsCredentialId: ID!): [Ec2AmiImportTask!]!
}
```

### Types

```graphql
type User {
  id: ID!
  tenantId: ID!
  email: String!
  name: String!
  createdAt: DateTime!
  updatedAt: DateTime!
}

type Tenant {
  id: ID!
  name: String!
  slug: String!
  createdAt: DateTime!
  updatedAt: DateTime!
  isActive: Boolean!
}

input RegisterTenantInput {
  name: String!
  slug: String!
  adminEmail: String!
  adminName: String!
  adminPassword: String!
}

input CreateUserInput {
  email: String!
  name: String!
  password: String!
}

type AwsCredential {
  id: ID!
  tenantId: ID!
  name: String!
  region: String!
  createdAt: DateTime!
  updatedAt: DateTime!
}

input CreateAwsCredentialInput {
  name: String!
  accessKeyId: String!
  secretAccessKey: String!
  region: String!
}

input UpdateAwsCredentialInput {
  name: String
  region: String
}

type Ec2AmiImportTask {
  id: ID!
  tenantId: ID!
  awsCredentialId: ID!
  importTaskId: String!
  status: String!
  statusMessage: String
  imageId: String
  architecture: String
  description: String
  hypervisor: String
  licenseType: String
  platform: String
  progress: String
  snapshotDetails: JSON
  tags: JSON
  createdAt: DateTime!
  updatedAt: DateTime!
}

input Ec2AmiImportTaskFilter {
  awsCredentialId: ID
  status: String
  importTaskId: String
  limit: Int
  offset: Int
}

scalar DateTime
scalar JSON
```

## Authentication

all authenticated requests must include a JWT token in the Authorization header:

```
Authorization: Bearer <token>
```

the token is obtained via the `login` mutation and contains:
- user_id (sub)
- tenant_id
- email
- expiration (24 hours)

the middleware automatically extracts the tenant context from the token and makes it available to all GraphQL resolvers.

## Error Handling

uses standard GraphQL error responses.

```json
{
  "errors": [
    {
      "message": "error message",
      "locations": [{ "line": 2, "column": 3 }],
      "path": ["fieldName"]
    }
  ],
  "data": null
}
```

## Usage Examples

### register tenant

```graphql
mutation {
  registerTenant(input: {
    name: "My Organization"
    slug: "my-org"
    adminEmail: "admin@example.com"
    adminName: "Admin User"
    adminPassword: "securepassword"
  }) {
    id
    name
    slug
    isActive
  }
}
```

### login

```graphql
mutation {
  login(email: "admin@example.com", password: "securepassword")
}
```

response:
```json
{
  "data": {
    "login": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

### get current user

```graphql
query {
  currentUser {
    id
    email
    name
    tenantId
  }
}
```

### create user (requires authentication)

```graphql
mutation {
  createUser(input: {
    email: "user@example.com"
    name: "New User"
    password: "password"
  }) {
    id
    email
    name
  }
}
```

### logout

```graphql
mutation {
  logout
}
```

### create AWS credential (requires authentication)

```graphql
mutation {
  createAwsCredential(input: {
    name: "Production Account"
    accessKeyId: "AKIAIOSFODNN7EXAMPLE"
    secretAccessKey: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
    region: "us-east-1"
  }) {
    id
    name
    region
    createdAt
  }
}
```

note: credentials are encrypted before storage and never returned in query responses.

### list AWS credentials (requires authentication)

```graphql
query {
  awsCredentials {
    id
    name
    region
    createdAt
    updatedAt
  }
}
```

### get AWS credential (requires authentication)

```graphql
query {
  awsCredential(id: "uuid-here") {
    id
    name
    region
    createdAt
    updatedAt
  }
}
```

### update AWS credential (requires authentication)

```graphql
mutation {
  updateAwsCredential(
    id: "uuid-here"
    input: {
      name: "Production Account (Updated)"
      region: "ap-northeast-1"
    }
  ) {
    id
    name
    region
    updatedAt
  }
}
```

### delete AWS credential (requires authentication)

```graphql
mutation {
  deleteAwsCredential(id: "uuid-here")
}
```

### sync EC2 AMI import tasks (requires authentication)

```graphql
mutation {
  syncImportImageTasks(awsCredentialId: "uuid-here") {
    id
    importTaskId
    status
    statusMessage
    imageId
    architecture
    platform
    progress
    createdAt
    updatedAt
  }
}
```

fetches all AMI import tasks from AWS API using the specified credential and stores them in the database. returns the list of synced tasks. existing tasks are updated (upsert).

### list EC2 AMI import tasks (requires authentication)

```graphql
query {
  ec2AmiImportTasks {
    id
    importTaskId
    status
    statusMessage
    imageId
    architecture
    platform
    progress
    createdAt
    updatedAt
  }
}
```

with filtering and pagination:

```graphql
query {
  ec2AmiImportTasks(filter: {
    awsCredentialId: "uuid-here"
    status: "completed"
    limit: 20
    offset: 0
  }) {
    id
    importTaskId
    status
    imageId
    progress
  }
}
```

### get EC2 AMI import task (requires authentication)

```graphql
query {
  ec2AmiImportTask(id: "uuid-here") {
    id
    awsCredentialId
    importTaskId
    status
    statusMessage
    imageId
    architecture
    description
    hypervisor
    licenseType
    platform
    progress
    snapshotDetails
    tags
    createdAt
    updatedAt
  }
}
```

## Versioning

current version: v1 (undefined)

due to GraphQL characteristics, avoid breaking changes and handle with field additions.

## Limitations

- query depth limit: (to be configured)
- rate limiting: (to be configured)
- max request size: (to be configured)

## Development Notes

### when changing schema

1. update this document
2. modify backend implementation
3. update frontend client code
4. notify both leads of impact

### testing

use GraphiQL IDE to test queries and mutations.

### documentation generation

(note here if planning to introduce tools)
