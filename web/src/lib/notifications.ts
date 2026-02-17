import React from "react";
import {
  notifications as base,
  type NotificationData,
} from "@mantine/notifications";
import { IconCheck, IconX } from "@tabler/icons-react";

function show(data: NotificationData) {
  return base.show({
    position: "top-right",
    radius: "md",
    autoClose: 10000,
    withBorder: true,
    style: {
      boxShadow: "none",
    },
    ...data,
  });
}

const notifications = {
  info: (message: React.ReactNode) => {
    show({
      title: "Info",
      color: "green",
      icon: React.createElement(IconCheck, { size: 18 }),
      message,
    });
  },
  error: (message: React.ReactNode) => {
    show({
      title: "Error",
      color: "red",
      icon: React.createElement(IconX, { size: 18 }),
      message,
    });
  },
};

export default notifications;
