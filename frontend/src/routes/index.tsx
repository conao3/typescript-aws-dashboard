import { createFileRoute } from "@tanstack/react-router";
import { Heading, Text } from "react-aria-components";
import { AppHeader, Sidebar } from "../components/layout";

export const Route = createFileRoute("/")({ component: Dashboard });

function Dashboard() {
  return (
    <div className="min-h-screen bg-gray-50">
      <AppHeader />
      <div className="flex h-[calc(100vh-4rem)]">
        <Sidebar />
        <main className="flex-1 overflow-auto">
          <div className="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
            <Heading className="text-3xl font-bold text-gray-900">
              Dashboard
            </Heading>
            <div className="mt-8 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
              <DashboardCard
                title="Total Users"
                value="0"
                description="Registered users"
              />
              <DashboardCard
                title="Active Sessions"
                value="0"
                description="Current active sessions"
              />
              <DashboardCard
                title="API Calls"
                value="0"
                description="Total API calls today"
              />
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}

function DashboardCard({
  title,
  value,
  description,
}: {
  title: string;
  value: string;
  description: string;
}) {
  return (
    <div className="rounded-lg bg-white p-6 shadow">
      <Heading level={3} className="text-sm font-medium text-gray-500">
        {title}
      </Heading>
      <Text className="mt-2 text-3xl font-semibold text-gray-900">{value}</Text>
      <Text className="mt-1 text-sm text-gray-600">{description}</Text>
    </div>
  );
}
