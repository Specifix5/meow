import { MESSAGES } from "./constants";

export const CreateErrorMessage = (err: Error): string => {
  return `${getString("ERROR_GENERIC")}\n\`\`\`diff\n- ${err}\n\`\`\``;
};

export interface Logger {
  info: (msg: string) => void;
  warn: (msg: string) => void;
  error: (msg: string) => void;
}

export interface GetString {
  (name: string): string | undefined;
}

export const getString: GetString = (name: string) => {
  const message = (MESSAGES as { [key: string]: string | string[] })[name];
  if (Array.isArray(message)) {
    return message[Math.floor((message.length - 1) * Math.random())];
  } else if (typeof message === "string") {
    return message;
  }
};

export const getLogger = (): Logger => ({
  info: (msg: string, name: string = "MASTER") => console.info(`[${name}] [INFO] ${new Date().toISOString()} - ${msg}`),
  warn: (msg: string, name: string = "MASTER") => console.warn(`[${name}] [WARN] ${new Date().toISOString()} - ${msg}`),
  error: (msg: string, name: string = "MASTER") =>
    console.error(`[${name}] [ERROR] ${new Date().toISOString()} - ${msg}`),
});
