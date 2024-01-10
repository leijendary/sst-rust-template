import { Api, StackContext, use } from "sst/constructs";
import { Database } from "../Database";
import { CustomerAuth } from "./CustomerAuth";

export function CustomerApi({ stack }: StackContext) {
  const { auth } = use(CustomerAuth);
  const database = use(Database);
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
          handler: "./api_v1_sample_seek.rs",
          description: "Customer: Seek pagination of sample records.",
        },
      },
      "GET /api/v1/samples/{id}": {
        function: {
          handler: "./api_v1_sample_get.rs",
          description: "Customer: Get a single sample record.",
        },
      },
      $default: {
        authorizer: "none",
        function: {
          handler: "./api_default.rs",
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
