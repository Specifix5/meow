import { MeowClient } from "../utils/nyan/client.js";

const errorHandler = (client: MeowClient, err: Error) => {
  client.logger.error("[Unhandled Error] " + err.message);
};

export default (client: MeowClient) => {
  process.on("uncaughtException", (error: Error) => void errorHandler(client, error));
  process.on("unhandledRejection", (error: Error) => void errorHandler(client, error));
  client.on("error", (error: Error) => void errorHandler(client, error));
  client.on("shardError", (error: Error) => void errorHandler(client, error));
};
