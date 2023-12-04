import { Api, StackContext, use } from "sst/constructs";
import { DatabaseStack } from "../DatabaseStack";
import { AdminAuthStack } from "./AdminAuthStack";

export function AdminApiStack({ stack }: StackContext) {
  const { auth } = use(AdminAuthStack);
  const database = use(DatabaseStack);
  const api = new Api(stack, "Admin", {
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
      "GET /api/admin/samples": {
        function: {
          handler: "functions/api/admin/samples/admin_sample_list.rs",
          description: "Admin: List of sample records.",
        },
      },
      "POST /api/admin/samples": {
        function: {
          handler: "functions/api/admin/samples/admin_sample_create.rs",
          description: "Admin: Create a sample record.",
        },
      },
      "GET /api/admin/samples/{id}": {
        function: {
          handler: "functions/api/admin/samples/admin_sample_get.rs",
          description: "Admin: Get a single sample record.",
        },
      },
      "PUT /api/admin/samples/{id}": {
        function: {
          handler: "functions/api/admin/samples/admin_sample_update.rs",
          description: "Admin: Update a specific single sample record.",
        },
      },
      "DELETE /api/admin/samples/{id}": {
        function: {
          handler: "functions/api/admin/samples/admin_sample_delete.rs",
          description: "Admin: Soft delete a specific single sample record.",
        },
      },
      $default: {
        authorizer: "none",
        function: {
          handler: "functions/api/default.rs",
          description: "Admin: Default route handler.",
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
