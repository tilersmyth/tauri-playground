import { listen } from "@tauri-apps/api/event";
import { Typography } from "antd";
import React, { useEffect, useState } from "react";

export const PortReading: React.FC = () => {
  const [reading, setReading] = useState<string>("");

  useEffect(() => {
    const unlisten = listen("port_reading", (v) =>
      setReading(v.payload as string)
    );

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return <Typography.Title level={5}>Reading: {reading}</Typography.Title>;
};
