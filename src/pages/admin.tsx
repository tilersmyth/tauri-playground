import { invoke } from "@tauri-apps/api";
import { Button, Typography } from "antd";
import React from "react";
import { useNavigate } from "react-router-dom";

export const AdminPage: React.FC = () => {
  const navigate = useNavigate();

  const logout = async () => {
    try {
      const res = await invoke<any>("logout");

      if (!res.is_logged_in) {
        navigate("/");
      }
    } catch (error) {
      console.log("err: ", error);
    }
  };

  return (
    <>
      <Typography.Title>ADMIN PAGE</Typography.Title>
      <br />
      <Button type="primary" onClick={logout}>
        logout
      </Button>
    </>
  );
};
