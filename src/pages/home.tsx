import { Button, Typography } from "antd";
import React from "react";
import { useNavigate } from "react-router-dom";
import { PortStatus } from "./status";

export const HomePage: React.FC = () => {
  const navigate = useNavigate();

  return (
    <>
      <Typography.Title>HOME PAGE</Typography.Title>
      <br />
      <br />
      <PortStatus />
      <br />
      <br />
      <Button type="primary" onClick={() => navigate("/about")}>
        Go to about
      </Button>
      <br />
    </>
  );
};
