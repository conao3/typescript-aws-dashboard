const TOKEN_KEY = 'auth_token'

export interface TokenPayload {
  sub: string
  tenant_id: string
  email: string
  exp: number
}

export function saveToken(token: string): void {
  localStorage.setItem(TOKEN_KEY, token)
}

export function getToken(): string | null {
  return localStorage.getItem(TOKEN_KEY)
}

export function removeToken(): void {
  localStorage.removeItem(TOKEN_KEY)
}

export function decodeToken(token: string): TokenPayload | null {
  try {
    const parts = token.split('.')
    if (parts.length !== 3) {
      return null
    }
    const payload = JSON.parse(atob(parts[1]))
    return payload as TokenPayload
  } catch {
    return null
  }
}

export function isTokenExpired(token: string): boolean {
  const payload = decodeToken(token)
  if (!payload) {
    return true
  }
  return Date.now() >= payload.exp * 1000
}

export function getTokenPayload(): TokenPayload | null {
  const token = getToken()
  if (!token) {
    return null
  }
  if (isTokenExpired(token)) {
    removeToken()
    return null
  }
  return decodeToken(token)
}
