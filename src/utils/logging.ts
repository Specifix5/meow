import { EMOJI, MESSAGES } from "./constants.js";

/**
 * Creates a uniform stylized error message meant to be shown to the user.
 * @param err Error message to pass.
 * @returns Error message in a string, meant for discord text formatting.
 */
export const CreateErrorMessage = (err: Error): string => {
  return `${EMOJI.ICON_DENY} ${getString("ERROR_GENERIC")}\n\`\`\`diff\n- ${err}\n\`\`\``;
};

/**
 * The whole client logger, with neat format.
 */
export interface Logger {
  info: (msg: string) => void;
  warn: (msg: string) => void;
  error: (msg: string) => void;
}

/**
 * Functional way to get strings that exists in MESSAGES.
 * Randomized strings from string arrays.
 */
export type GetString = (name: string) => string | undefined;

export const getString: GetString = (name: string) => {
  const message = (MESSAGES as Record<string, string | string[]>)[name];
  if (Array.isArray(message)) {
    return message[Math.floor((message.length - 1) * Math.random())];
  } else if (typeof message === "string") {
    return message;
  }
};

export const getLogger = (): Logger => ({
  info: (msg: string, name = "MASTER") =>
    console.info(`[${name}] [INFO] ${new Date().toISOString()} - ${msg}`),
  warn: (msg: string, name = "MASTER") =>
    console.warn(`[${name}] [WARN] ${new Date().toISOString()} - ${msg}`),
  error: (msg: string, name = "MASTER") =>
    console.error(`[${name}] [ERROR] ${new Date().toISOString()} - ${msg}`),
});
