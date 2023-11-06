import { SSTConfig } from "sst";
import ApiStack from "./stacks/ApiStack";
import AuthStack from "./stacks/AuthStack";

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
    app.stack(AuthStack).stack(ApiStack);
  },
} satisfies SSTConfig;
