create schema if not exists dashboard;

create table dashboard.tenants (
  id uuid primary key default gen_random_uuid(),
  name text not null,
  slug text not null unique,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  is_active boolean not null default true
);

create index idx_tenants_slug on dashboard.tenants(slug);
create index idx_tenants_is_active on dashboard.tenants(is_active);

create table dashboard.users (
  id uuid primary key default gen_random_uuid(),
  tenant_id uuid not null references dashboard.tenants(id) on delete cascade,
  email text not null,
  name text not null,
  password_hash text not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index idx_users_tenant on dashboard.users(tenant_id);
create unique index idx_users_tenant_email on dashboard.users(tenant_id, email);

create table dashboard.aws_credentials (
  id uuid primary key default gen_random_uuid(),
  tenant_id uuid not null references dashboard.tenants(id) on delete cascade,
  name text not null,
  access_key_id_encrypted text not null,
  secret_access_key_encrypted text not null,
  region text not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index idx_aws_credentials_tenant on dashboard.aws_credentials(tenant_id);

create table dashboard.ec2_ami_import_tasks (
  id uuid primary key default gen_random_uuid(),
  tenant_id uuid not null references dashboard.tenants(id) on delete cascade,
  aws_credential_id uuid not null references dashboard.aws_credentials(id) on delete cascade,
  import_task_id text not null,
  status text not null,
  status_message text,
  image_id text,
  architecture text,
  description text,
  hypervisor text,
  license_type text,
  platform text,
  progress text,
  snapshot_details jsonb,
  tags jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index idx_ec2_ami_import_tasks_tenant on dashboard.ec2_ami_import_tasks(tenant_id);
create index idx_ec2_ami_import_tasks_aws_credential on dashboard.ec2_ami_import_tasks(aws_credential_id);
create unique index idx_ec2_ami_import_tasks_tenant_import_task on dashboard.ec2_ami_import_tasks(tenant_id, import_task_id);
