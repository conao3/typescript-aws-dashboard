import { createContext, useContext, useState, useEffect } from "react";
import {
  removeToken,
  saveToken,
  getTokenPayload,
  type TokenPayload,
} from "./token";

interface AuthContextValue {
  isAuthenticated: boolean;
  user: TokenPayload | null;
  login: (token: string) => void;
  logout: () => void;
  isLoading: boolean;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<TokenPayload | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const payload = getTokenPayload();
    setUser(payload);
    setIsLoading(false);
  }, []);

  const login = (token: string) => {
    saveToken(token);
    const payload = getTokenPayload();
    setUser(payload);
  };

  const logout = () => {
    removeToken();
    setUser(null);
  };

  return (
    <AuthContext.Provider
      value={{
        isAuthenticated: user !== null,
        user,
        login,
        logout,
        isLoading,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within AuthProvider");
  }
  return context;
}
