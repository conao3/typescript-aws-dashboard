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
}
```

### Mutation

```graphql
type Mutation {
  login(email: String!, password: String!): String!
  logout: Boolean!
  registerTenant(input: RegisterTenantInput!): Tenant!
  createUser(input: CreateUserInput!): User!
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

scalar DateTime
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
