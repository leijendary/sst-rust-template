import { SSTConfig } from "sst";
import { DatabaseStack } from "./stacks/DatabaseStack";
import { AdminApiStack } from "./stacks/admin/AdminApiStack";
import { AdminAuthStack } from "./stacks/admin/AdminAuthStack";
import { CustomerApiStack } from "./stacks/customer/CustomerApiStack";
import { CustomerAuthStack } from "./stacks/customer/CustomerAuthStack";

export default {
  config(_input) {
    return {
      name: "sst-rust-template",
      region: "eu-central-1",
    };
  },
  stacks(app) {
    if (app.stage === "dev") {
      app.setDefaultRemovalPolicy("destroy");
    }

    app.setDefaultFunctionProps({
      architecture: "arm_64",
      logRetention: app.stage === "prod" ? "three_months" : "one_week",
      runtime: "rust",
      timeout: "30 seconds",
    });
    app
      .stack(DatabaseStack)
      .stack(CustomerAuthStack)
      .stack(CustomerApiStack)
      .stack(AdminAuthStack)
      .stack(AdminApiStack);
  },
} satisfies SSTConfig;
