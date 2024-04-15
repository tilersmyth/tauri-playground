import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Navigate, Outlet } from "react-router-dom";

interface AuthState {
  loading: boolean;
  auth: boolean;
}

export const AuthRoutes = () => {
  const [isAuth, setIsAuth] = useState<AuthState>({
    loading: true,
    auth: false,
  });

  useEffect(() => {
    (async () => {
      try {
        const res = await invoke<any>("get_auth_state");
        setIsAuth({ loading: false, auth: res.logged_in });
      } catch (error) {
        console.log("Error: ", error);
      }
    })();
  }, []);

  if (isAuth.loading) {
    return null;
  }

  return isAuth.auth ? <Outlet /> : <Navigate to="/" />;
};

export const NonAuthRoutes = () => {
  const [isAuth, setIsAuth] = useState<AuthState>({
    loading: true,
    auth: false,
  });

  useEffect(() => {
    (async () => {
      try {
        const res = await invoke<any>("get_auth_state");
        setIsAuth({ loading: false, auth: res.logged_in });
      } catch (error) {
        console.log("Error: ", error);
      }
    })();
  }, []);

  if (isAuth.loading) {
    return null;
  }

  return !isAuth.auth ? <Outlet /> : <Navigate to="/admin" />;
};
