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
      /* authorizer: "jwt", */
      function: {
        bind: [...Object.values(database)],
      },
    },
    routes: {
      "GET /api/admin/samples": "functions/api/admin/samples/sample_list.rs",
      /* "GET /api/admin/samples/{id}": "functions/api/admin/samples/get",
      "POST /api/admin/samples": "functions/api/admin/samples/save",
      "PUT /api/admin/samples/{id}": "functions/api/admin/samples/update",
      "DELETE /api/admin/samples/{id}": "functions/api/admin/samples/delete",
      $default: {
        authorizer: "none",
        function: "functions/api/default.go",
      }, */
    },
  });
  auth.attachPermissionsForAuthUsers(stack, [api]);

  stack.addOutputs({
    ApiEndpoint: api.url,
  });

  return { api };
}
