import { SSTConfig } from "sst";
import AdminApiStack from "./stacks/AdminApiStack";
import AdminAuthStack from "./stacks/AdminAuthStack";
import CustomerApiStack from "./stacks/CustomerApiStack";
import CustomerAuthStack from "./stacks/CustomerAuthStack";
import DatabaseStack from "./stacks/DatabaseStack";

export default {
  config(_input) {
    return {
      name: "go-sst-template",
      region: "eu-central-1",
    };
  },
  stacks(app) {
    app.setDefaultFunctionProps({
      architecture: "arm_64",
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
