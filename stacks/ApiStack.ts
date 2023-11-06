import { Api, StackContext, use } from "sst/constructs";
import DatabaseConfig from "../config/DatabaseConfig";
import AuthStack from "./AuthStack";

export default function ApiStack({ stack }: StackContext) {
  const { auth } = use(AuthStack);
  const database = DatabaseConfig(stack);
  const api = new Api(stack, "api", {
    authorizers: {
      jwt: {
        type: "user_pool",
        userPool: {
          id: auth.userPoolId,
          clientIds: [auth.userPoolClientId],
        },
      },
    },
    defaults: {
      authorizer: "jwt",
      function: {
        bind: [...database],
      },
    },
    routes: {
      "GET /api/v1/samples": "functions/v1/samples/get",
      "POST /api/v1/samples": "functions/v1/samples/save",
      $default: {
        authorizer: "none",
        function: "functions/default",
      },
    },
  });
  auth.attachPermissionsForAuthUsers(stack, [api]);

  stack.addOutputs({
    ApiEndpoint: api.url,
  });

  return { api };
}
