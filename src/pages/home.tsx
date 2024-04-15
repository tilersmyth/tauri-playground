import { invoke } from "@tauri-apps/api";
import { Button, Typography } from "antd";
import React from "react";
import { useNavigate } from "react-router-dom";

export const HomePage: React.FC = () => {
  const navigate = useNavigate();

  const login = async () => {
    try {
      const res = await invoke<any>("login");

      if (res.logged_in) {
        navigate("/admin");
      }
    } catch (error) {
      console.log("err: ", error);
    }
  };

  return (
    <>
      <Typography.Title>LOGIN PAGE</Typography.Title>
      <Button type="primary" onClick={login}>
        login
      </Button>
      <br />
      <br />
      <Button type="primary" onClick={() => navigate("/admin")}>
        Try going to admin!
      </Button>
      <br />
    </>
  );
};
