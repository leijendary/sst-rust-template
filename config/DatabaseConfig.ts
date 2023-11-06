import { Config, Stack } from "sst/constructs";

export default function DatabaseConfig(stack: Stack) {
  const username = new Config.Secret(stack, "DB_USERNAME");
  const password = new Config.Secret(stack, "DB_PASSWORD");
  const url = new Config.Secret(stack, "DB_URL");
  const name = new Config.Secret(stack, "DB_NAME");
  const sslMode = new Config.Secret(stack, "DB_SSL_MODE");

  return [username, password, url, name, sslMode];
}
