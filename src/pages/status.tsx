import { listen } from "@tauri-apps/api/event";
import { Typography } from "antd";
import React, { useCallback, useEffect, useState } from "react";

export const PortStatus: React.FC = () => {
  const [port, setPort] = useState<any>(null);

  const handleWeight = useCallback((e: any) => {
    setPort(e.payload);
  }, []);

  useEffect(() => {
    const unlisten = listen("port_status", handleWeight);

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const p = JSON.parse(port);

  return (
    <Typography.Title level={3} type={!p ? "danger" : "success"}>
      Port Status: {!p ? "not connected" : `connected on ${p.port_name}`}
    </Typography.Title>
  );
};
