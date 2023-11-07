import { Api, StackContext, use } from "sst/constructs";
import AdminAuthStack from "./AdminAuthStack";
import DatabaseStack from "./DatabaseStack";

export default function AdminApiStack({ stack }: StackContext) {
  const { auth } = use(AdminAuthStack);
  const database = use(DatabaseStack);
  const api = new Api(stack, "admin", {
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
      "GET /api/admin/samples": "functions/admin/samples/get",
      "POST /api/admin/samples": "functions/admin/samples/save",
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
