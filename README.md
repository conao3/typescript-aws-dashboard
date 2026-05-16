# typescript-aws-dashboard

a multi-tenant SaaS dashboard providing AWS management features missing from the AWS Console.

## Overview

this project fills gaps in AWS Console functionality by providing:

- **EC2 AMI import tracking**: view and monitor AMI import tasks not visible in AWS Console
- **service-specific modules**: organized by AWS service for targeted functionality
- **multi-tenant architecture**: secure isolation for multiple organizations

see [docs/requirements.md](./docs/requirements.md) for detailed requirements and scope.

## Setup

```bash
nix develop
make setup
```

## Development

Start PostgreSQL database:

```bash
make db-up
```

Start backend server:

```bash
make dev-backend
```

GraphiQL IDE: http://localhost:17231/admin/graphiql

Start frontend development server:

```bash
make dev
```

Frontend: http://localhost:17232

### Database Commands

```bash
make db-up           # start postgres
make db-down         # stop postgres
make db-logs         # show logs
make db-reset        # reset database
make db-migrate-dry  # preview schema changes
make db-migrate      # apply schema changes
```

Database connection:

- Host: localhost
- Port: 5432
- User: dashboard
- Password: dashboard
- Database: dashboard
- Schema: dashboard

## Build

```bash
make build
```

## Test

```bash
make test
```

## Format

```bash
make fmt
```

## Lint

```bash
make lint
```

## Deployment

### using Docker Compose

create `.env` file from template:

```bash
cp .env.example .env
```

edit `.env` and set production values, especially:

- `POSTGRES_PASSWORD`: strong password for database
- `JWT_SECRET`: random secret key for JWT token signing

build and start services:

```bash
docker compose -f docker-compose.prod.yml up -d
```

the backend container automatically:

1. waits for PostgreSQL to be ready
2. applies database schema using psqldef
3. starts the backend server

services will be available at:

- backend: http://localhost:17231
- GraphiQL: http://localhost:17231/admin/graphiql
- frontend: http://localhost:17232

view logs:

```bash
docker compose -f docker-compose.prod.yml logs -f
```

stop services:

```bash
docker compose -f docker-compose.prod.yml down
```

or use Makefile commands:

```bash
make docker-build    # build images
make docker-up       # start services
make docker-logs     # view logs
make docker-down     # stop services
```

### environment variables

required environment variables for production:

- `POSTGRES_PASSWORD`: PostgreSQL password
- `JWT_SECRET`: secret key for JWT token signing
- `RUST_LOG`: log level (default: info)
- `GRAPHQL_URL`: GraphQL endpoint URL (default: http://backend:17231/graphql)

## Documentation

For detailed documentation, see the [docs](./docs) directory:

- [Requirements](./docs/requirements.md) - project requirements and scope
- [Claude Code Coordination Guide](./docs/coordination.md) - guidelines for multiple Claude Code instances
- [Current Tasks](./docs/tasks.md) - task tracking and assignments
- [Architecture](./docs/architecture.md) - system architecture and multi-tenancy design
- [Multi-Tenancy Guide](./docs/multi-tenancy.md) - multi-tenancy implementation details
- [Database Guide](./docs/database.md) - database setup and sqldef schema management
- [Development Guide](./docs/development.md) - development environment and workflow
- [API Specification](./docs/api.md) - GraphQL API specification
