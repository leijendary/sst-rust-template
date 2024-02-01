import { Template } from "aws-cdk-lib/assertions";
import { App, getStack } from "sst/constructs";
import { initProject } from "sst/project";
import { test } from "vitest";
import { Database } from "../../stack/Database";
import { CustomerApi } from "../../stack/customer/CustomerApi";
import { CustomerAuth } from "../../stack/customer/CustomerAuth";

test("Created Customer api gateway", async () => {
  await initProject({});
  const app = new App({ mode: "deploy" });
  app.stack(Database);
  app.stack(CustomerAuth);
  app.stack(CustomerApi);

  const template = Template.fromStack(getStack(CustomerApi));

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
});
