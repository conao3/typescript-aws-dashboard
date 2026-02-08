import { ApolloClient, InMemoryCache, HttpLink, from } from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { onError } from "@apollo/client/link/error";
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

const errorLink = onError(({ graphQLErrors, networkError }) => {
  if (graphQLErrors) {
    graphQLErrors.forEach(({ message, extensions }) => {
      console.error(`[GraphQL error]: Message: ${message}`);
      if (extensions?.code === "UNAUTHENTICATED") {
        removeToken();
        window.location.href = "/login";
      }
    });
  }
  if (networkError) {
    console.error(`[Network error]: ${networkError}`);
    if ("statusCode" in networkError && networkError.statusCode === 401) {
      removeToken();
      window.location.href = "/login";
    }
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
