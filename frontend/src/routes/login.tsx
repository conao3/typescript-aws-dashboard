import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  Button,
  TextField,
  Label,
  Input,
  FieldError,
  Form,
  Heading,
  Text,
} from "react-aria-components";
import { useState, useEffect } from "react";
import { useAuth } from "../lib/auth";

export const Route = createFileRoute("/login")({ component: Login });

const GRAPHQL_ENDPOINT = "http://localhost:17231/graphql";

async function loginMutation(email: string, password: string): Promise<string> {
  const response = await fetch(GRAPHQL_ENDPOINT, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      query: `
        mutation Login($email: String!, $password: String!) {
          login(email: $email, password: $password)
        }
      `,
      variables: { email, password },
    }),
  });

  if (!response.ok) {
    throw new Error(`HTTP error: ${response.status}`);
  }

  const result = await response.json();

  if (result.errors) {
    throw new Error(result.errors[0]?.message || "Login failed");
  }

  return result.data.login;
}

function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const { isAuthenticated, login } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    if (isAuthenticated) {
      navigate({ to: "/" });
    }
  }, [isAuthenticated, navigate]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setIsLoading(true);

    try {
      const token = await loginMutation(email, password);
      login(token);
      navigate({ to: "/" });
    } catch (err) {
      setError(err instanceof Error ? err.message : "Login failed");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50 px-4 py-12 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8">
        <div>
          <Heading className="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900">
            Sign in to your account
          </Heading>
          <Text className="mt-2 text-center text-sm text-gray-600">
            AWS Dashboard - Multi-tenant Application
          </Text>
        </div>
        <Form className="mt-8 space-y-6" onSubmit={handleSubmit}>
          <div className="space-y-4 rounded-md shadow-sm">
            <TextField
              name="email"
              type="email"
              isRequired
              className="space-y-1"
              value={email}
              onChange={setEmail}
            >
              <Label className="text-sm font-medium text-gray-700">
                Email address
              </Label>
              <Input
                autoComplete="email"
                className="block w-full rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-400 focus:border-blue-500 focus:outline-none focus:ring-blue-500"
                placeholder="you@example.com"
              />
              <FieldError className="text-sm text-red-600" />
            </TextField>

            <TextField
              name="password"
              type="password"
              isRequired
              className="space-y-1"
              value={password}
              onChange={setPassword}
            >
              <Label className="text-sm font-medium text-gray-700">
                Password
              </Label>
              <Input
                autoComplete="current-password"
                className="block w-full rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-400 focus:border-blue-500 focus:outline-none focus:ring-blue-500"
                placeholder="••••••••"
              />
              <FieldError className="text-sm text-red-600" />
            </TextField>
          </div>

          {error && (
            <div className="rounded-md bg-red-50 p-4">
              <Text className="text-sm text-red-800">{error}</Text>
            </div>
          )}

          <Button
            type="submit"
            isDisabled={isLoading}
            className="group relative flex w-full justify-center rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white hover:bg-blue-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-50"
          >
            {isLoading ? "Signing in..." : "Sign in"}
          </Button>
        </Form>
      </div>
    </div>
  );
}
