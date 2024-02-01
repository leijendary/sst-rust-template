import { Template } from "aws-cdk-lib/assertions";
import { App, getStack } from "sst/constructs";
import { initProject } from "sst/project";
import { test } from "vitest";
import { AdminAuth } from "../../stack/admin/AdminAuth";

test("Created Admin user pool", async () => {
  await initProject({});
  const app = new App({ mode: "deploy" });
  app.stack(AdminAuth);

  const template = Template.fromStack(getStack(AdminAuth));

  template.hasResourceProperties("AWS::Cognito::UserPool", {
    AutoVerifiedAttributes: ["email"],
    UsernameAttributes: ["email"],
    UsernameConfiguration: {
      CaseSensitive: false,
    },
  });
});
