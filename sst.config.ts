import { SSTConfig } from "sst";
import AdminApiStack from "./stacks/AdminApiStack";
import AdminAuthStack from "./stacks/AdminAuthStack";
import CustomerApiStack from "./stacks/CustomerApiStack";
import CustomerAuthStack from "./stacks/CustomerAuthStack";
import DatabaseStack from "./stacks/DatabaseStack";

export default {
  config(_input) {
    return {
      name: "sst-go-template",
      region: "eu-central-1",
    };
  },
  stacks(app) {
    app.setDefaultFunctionProps({
      architecture: "arm_64",
      logRetention: app.stage === "prod" ? "three_months" : "one_week",
      runtime: "go",
    });
    app
      .stack(DatabaseStack)
      .stack(CustomerAuthStack)
      .stack(CustomerApiStack)
      .stack(AdminAuthStack)
      .stack(AdminApiStack);
  },
} satisfies SSTConfig;
