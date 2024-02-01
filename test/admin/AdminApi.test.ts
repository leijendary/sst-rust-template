import { Template } from "aws-cdk-lib/assertions";
import { App, getStack } from "sst/constructs";
import { initProject } from "sst/project";
import { test } from "vitest";
import { Database } from "../../stack/Database";
import { AdminApi } from "../../stack/admin/AdminApi";
import { AdminAuth } from "../../stack/admin/AdminAuth";

test("Created Admin api gateway", async () => {
  await initProject({});
  const app = new App({ mode: "deploy" });
  app.stack(Database);
  app.stack(AdminAuth);
  app.stack(AdminApi);

  const template = Template.fromStack(getStack(AdminApi));
  template.hasResourceProperties("AWS::ApiGatewayV2::Api", {
    CorsConfiguration: {
      AllowCredentials: false,
      AllowHeaders: ["*"],
      AllowMethods: ["*"],
      AllowOrigins: ["*"],
    },
    ProtocolType: "HTTP",
  });
  template.hasResourceProperties("AWS::ApiGatewayV2::Authorizer", {
    AuthorizerType: "JWT",
    IdentitySource: ["$request.header.Authorization"],
  });
  template.hasResourceProperties("AWS::ApiGatewayV2::Route", {
    AuthorizationType: "JWT",
    RouteKey: "GET /api/admin/samples",
  });
  template.hasResourceProperties("AWS::ApiGatewayV2::Route", {
    AuthorizationType: "JWT",
    RouteKey: "POST /api/admin/samples",
  });
  template.hasResourceProperties("AWS::ApiGatewayV2::Route", {
    AuthorizationType: "JWT",
    RouteKey: "GET /api/admin/samples/{id}",
  });
  template.hasResourceProperties("AWS::ApiGatewayV2::Route", {
    AuthorizationType: "JWT",
    RouteKey: "PUT /api/admin/samples/{id}",
  });
  template.hasResourceProperties("AWS::ApiGatewayV2::Route", {
    AuthorizationType: "JWT",
    RouteKey: "DELETE /api/admin/samples/{id}",
  });
  template.hasResourceProperties("AWS::ApiGatewayV2::Route", {
    AuthorizationType: "NONE",
    RouteKey: "$default",
  });
});
