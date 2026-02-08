import { Link, useNavigate } from "@tanstack/react-router";
import { Button, Header, Group, Text } from "react-aria-components";
import { useAuth } from "../../lib/auth";

export function AppHeader() {
  const { isAuthenticated, user, logout } = useAuth();
  const navigate = useNavigate();

  const handleLogout = () => {
    logout();
    navigate({ to: "/login" });
  };

  return (
    <Header className="bg-white shadow">
      <div className="mx-auto max-w-7xl px-4 py-4 sm:px-6 lg:px-8">
        <Group className="flex items-center justify-between">
          <Group className="flex items-center gap-8">
            <Link to="/" className="text-2xl font-bold text-gray-900">
              AWS Dashboard
            </Link>
            <nav className="flex gap-4">
              <Link
                to="/"
                className="text-sm font-medium text-gray-700 hover:text-gray-900"
              >
                Home
              </Link>
            </nav>
          </Group>
          <Group className="flex items-center gap-4">
            {isAuthenticated ? (
              <>
                <Text className="text-sm text-gray-700">{user?.email}</Text>
                <Button
                  onPress={handleLogout}
                  className="rounded-md bg-gray-600 px-4 py-2 text-sm font-semibold text-white hover:bg-gray-700"
                >
                  Logout
                </Button>
              </>
            ) : (
              <Link to="/login">
                <Button className="rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white hover:bg-blue-700">
                  Login
                </Button>
              </Link>
            )}
          </Group>
        </Group>
      </div>
    </Header>
  );
}
