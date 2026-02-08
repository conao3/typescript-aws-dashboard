import { Link } from "@tanstack/react-router";
import { Heading, ListBox, ListBoxItem, Text } from "react-aria-components";
import { Home, Settings, Users } from "lucide-react";

export function Sidebar() {
  return (
    <aside className="w-64 bg-gray-900 text-white">
      <div className="flex h-full flex-col">
        <div className="p-4">
          <Heading className="text-xl font-bold">Navigation</Heading>
        </div>
        <nav className="flex-1 px-2 py-4">
          <ListBox
            aria-label="Navigation"
            className="space-y-1"
            selectionMode="single"
          >
            <ListBoxItem
              id="dashboard"
              textValue="Dashboard"
              className="outline-none"
            >
              <Link
                to="/"
                className="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium hover:bg-gray-800"
                activeProps={{
                  className: "bg-gray-800",
                }}
              >
                <Home size={20} aria-hidden="true" />
                <Text>Dashboard</Text>
              </Link>
            </ListBoxItem>
            <ListBoxItem id="users" textValue="Users" className="outline-none">
              <Link
                to="/users"
                className="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium hover:bg-gray-800"
                activeProps={{
                  className: "bg-gray-800",
                }}
              >
                <Users size={20} aria-hidden="true" />
                <Text>Users</Text>
              </Link>
            </ListBoxItem>
            <ListBoxItem
              id="settings"
              textValue="Settings"
              className="outline-none"
            >
              <Link
                to="/settings"
                className="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium hover:bg-gray-800"
                activeProps={{
                  className: "bg-gray-800",
                }}
              >
                <Settings size={20} aria-hidden="true" />
                <Text>Settings</Text>
              </Link>
            </ListBoxItem>
          </ListBox>
        </nav>
      </div>
    </aside>
  );
}
