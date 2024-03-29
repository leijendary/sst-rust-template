import { Api, StackContext, use } from "sst/constructs";
import { Database } from "../Database";
import { AdminAuth } from "./AdminAuth";

export function AdminApi({ stack }: StackContext) {
  const { auth } = use(AdminAuth);
  const database = use(Database);
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
          handler: "./api_admin_sample_page.rs",
          description: "Admin: Page of sample records.",
        },
      },
      "POST /api/admin/samples": {
        function: {
          handler: "./api_admin_sample_create.rs",
          description: "Admin: Create a sample record.",
        },
      },
      "GET /api/admin/samples/{id}": {
        function: {
          handler: "./api_admin_sample_get.rs",
          description: "Admin: Get a single sample record.",
        },
      },
      "PUT /api/admin/samples/{id}": {
        function: {
          handler: "./api_admin_sample_update.rs",
          description: "Admin: Update a specific single sample record.",
        },
      },
      "DELETE /api/admin/samples/{id}": {
        function: {
          handler: "./api_admin_sample_delete.rs",
          description: "Admin: Soft delete a specific single sample record.",
        },
      },
      $default: {
        authorizer: "none",
        function: {
          handler: "./api_default.rs",
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
