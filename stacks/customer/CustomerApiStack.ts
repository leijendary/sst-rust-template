import { Api, StackContext, use } from "sst/constructs";
import { DatabaseStack } from "../DatabaseStack";
import { CustomerAuthStack } from "./CustomerAuthStack";

export function CustomerApiStack({ stack }: StackContext) {
  const { auth } = use(CustomerAuthStack);
  const database = use(DatabaseStack);
  const api = new Api(stack, "Customer", {
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
        bind: [...Object.values(database)],
      },
    },
    routes: {
      "GET /api/v1/samples": {
        function: {
          handler: "functions/api/v1/samples/sample_seek.rs",
          description: "Customer: Seek pagination of sample records.",
        },
      },
      "GET /api/v1/samples/{id}": {
        function: {
          handler: "functions/api/v1/samples/sample_get.rs",
          description: "Customer: Get a single sample record.",
        },
      },
      $default: {
        authorizer: "none",
        function: {
          handler: "functions/api/default.rs",
          description: "Customer: Default route handler.",
        },
      },
    },
  });
  auth.attachPermissionsForAuthUsers(stack, [api]);

  stack.addOutputs({
    ApiEndpoint: api.url,
  });

  return { api };
}
