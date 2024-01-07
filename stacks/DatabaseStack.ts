import { Config, StackContext } from "sst/constructs";

export function DatabaseStack({ stack }: StackContext) {
  const url = new Config.Secret(stack, "DATABASE_URL");

  return { url };
}
