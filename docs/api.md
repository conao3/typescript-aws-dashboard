# API Specification

this document defines the GraphQL API specification.

## Endpoints

- **development**: http://localhost:17231/graphql
- **GraphiQL IDE**: http://localhost:17231/admin/graphiql

## Schema

### Query

```graphql
type Query {
  # TODO: define queries
}
```

### Mutation

```graphql
type Mutation {
  # TODO: define mutations
}
```

### Types

```graphql
# TODO: type definitions
```

## Authentication

(to be implemented)

## Error Handling

uses standard GraphQL error responses.

```json
{
  "errors": [
    {
      "message": "error message",
      "locations": [{"line": 2, "column": 3}],
      "path": ["fieldName"]
    }
  ],
  "data": null
}
```

## Usage Examples

### query example

```graphql
# TODO: add after implementation
```

### mutation example

```graphql
# TODO: add after implementation
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
