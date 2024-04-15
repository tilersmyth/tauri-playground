import { Button, Typography } from "antd";
import React from "react";
import { useNavigate } from "react-router-dom";

export const AboutPage: React.FC = () => {
  const navigate = useNavigate();

  return (
    <>
      <Typography.Title>ABOUT PAGE</Typography.Title>
      <br />
      <Button type="primary" onClick={() => navigate("/")}>
        Go home
      </Button>
    </>
  );
};
