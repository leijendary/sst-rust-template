import { Tags } from "aws-cdk-lib/core";
import path from "path";
import { SSTConfig } from "sst";
import { App } from "sst/constructs";
import { Database } from "./stack/Database";
import { AdminApi } from "./stack/admin/AdminApi";
import { AdminAuth } from "./stack/admin/AdminAuth";
import { CustomerApi } from "./stack/customer/CustomerApi";
import { CustomerAuth } from "./stack/customer/CustomerAuth";

export default {
  config(_input) {
    return {
      name: "sst-rust-template",
      region: "eu-central-1",
    };
  },
  stacks(app) {
    if (app.stage !== "prod") {
      app.setDefaultRemovalPolicy("destroy");
    }

    resourceTags(app);
    functionDefaults(app);

    app.stack(Database).stack(CustomerAuth).stack(CustomerApi).stack(AdminAuth).stack(AdminApi);
  },
} satisfies SSTConfig;

function resourceTags(app: App) {
  const tags = Tags.of(app);
  tags.add("sst:region", app.region);
}

function functionDefaults(app: App) {
  app.setDefaultFunctionProps({
    architecture: "arm_64",
    functionName: ({ stack, functionProps }) => `${stack.stackName}-${path.parse(functionProps.handler!!).name}`,
    logRetention: app.stage === "prod" ? "three_months" : "one_week",
    runtime: "rust",
    timeout: "1 minute",
  });
}
