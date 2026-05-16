import { ApolloClient, InMemoryCache, HttpLink, from } from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { onError } from "@apollo/client/link/error";
import { CombinedGraphQLErrors, ServerError } from "@apollo/client/errors";
import { getToken, removeToken } from "./auth";

const httpLink = new HttpLink({
  uri: "http://localhost:17231/graphql",
});

const authLink = setContext((_, { headers }) => {
  const token = getToken();
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : "",
    },
  };
});

const errorLink = onError(({ error }) => {
  if (CombinedGraphQLErrors.is(error)) {
    error.errors.forEach(({ message, extensions }) => {
      console.error(`[GraphQL error]: Message: ${message}`);
      if (extensions?.code === "UNAUTHENTICATED") {
        removeToken();
        window.location.href = "/login";
      }
    });
  } else if (ServerError.is(error)) {
    console.error(`[Network error]: ${error}`);
    if (error.statusCode === 401) {
      removeToken();
      window.location.href = "/login";
    }
  } else if (error) {
    console.error(`[Network error]: ${error}`);
  }
});

export function createApolloClient() {
  return new ApolloClient({
    link: from([errorLink, authLink, httpLink]),
    cache: new InMemoryCache(),
    defaultOptions: {
      watchQuery: {
        fetchPolicy: "cache-and-network",
      },
    },
  });
}
