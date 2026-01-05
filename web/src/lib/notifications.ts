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
    ...data,
  });
}

const notifications = {
  info: (message: string) => {
    show({
      title: "Info",
      icon: React.createElement(IconCheck, { size: 18 }),
      message,
    });
  },
  error: (message: string) => {
    show({
      title: "Error",
      color: "red",
      icon: React.createElement(IconX, { size: 18 }),
      message,
    });
  },
};

export default notifications;
