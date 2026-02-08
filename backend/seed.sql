insert into dashboard.tenants (id, name, slug) values
  ('00000000-0000-0000-0000-000000000001', 'Test Tenant 1', 'test1'),
  ('00000000-0000-0000-0000-000000000002', 'Test Tenant 2', 'test2')
on conflict do nothing;

insert into dashboard.users (id, tenant_id, email, name, password_hash) values
  (
    '10000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000001',
    'user1@test1.example.com',
    'Test User 1',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYpe0E6m.Uu'
  ),
  (
    '10000000-0000-0000-0000-000000000002',
    '00000000-0000-0000-0000-000000000002',
    'user1@test2.example.com',
    'Test User 2',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYpe0E6m.Uu'
  )
on conflict do nothing;
