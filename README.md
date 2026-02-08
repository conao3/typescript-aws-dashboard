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
