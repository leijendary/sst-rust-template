import { Api, StackContext, use } from "sst/constructs";
import CustomerAuthStack from "./CustomerAuthStack";
import DatabaseStack from "./DatabaseStack";

export default function CustomerApiStack({ stack }: StackContext) {
  const { auth } = use(CustomerAuthStack);
  const database = use(DatabaseStack);
  const api = new Api(stack, "customer", {
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
        function: "functions/default.go",
      },
    },
  });
  auth.attachPermissionsForAuthUsers(stack, [api]);

  stack.addOutputs({
    ApiEndpoint: api.url,
  });

  return { api };
}
