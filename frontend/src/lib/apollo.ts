import { ApolloClient, InMemoryCache, HttpLink } from '@apollo/client'

export function createApolloClient() {
  return new ApolloClient({
    link: new HttpLink({
      uri: 'http://localhost:17231/graphql',
    }),
    cache: new InMemoryCache(),
  })
}
