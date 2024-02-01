import { Template } from "aws-cdk-lib/assertions";
import { App, getStack } from "sst/constructs";
import { initProject } from "sst/project";
import { test } from "vitest";
import { CustomerAuth } from "../../stack/customer/CustomerAuth";

test("Created Customer user pool", async () => {
  await initProject({});
  const app = new App({ mode: "deploy" });
  app.stack(CustomerAuth);

  const template = Template.fromStack(getStack(CustomerAuth));

  template.hasResourceProperties("AWS::Cognito::UserPool", {
    AdminCreateUserConfig: {
      AllowAdminCreateUserOnly: false,
    },
    UsernameConfiguration: {
      CaseSensitive: false,
    },
  });
});
