import React from "react";
import {
  notifications as base,
  type NotificationData,
} from "@mantine/notifications";
import { IconX } from "@tabler/icons-react";

function show(data: NotificationData) {
  return base.show({
    position: "top-right",
    radius: "md",
    autoClose: 10000,
    ...data,
  });
}

const notifications = {
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
