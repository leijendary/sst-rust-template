import { CfnUserPoolGroup } from "aws-cdk-lib/aws-cognito";
import { Cognito, Function, StackContext } from "sst/constructs";

export default function AuthStack({ stack }: StackContext) {
  const postConfirmation = new Function(stack, "CognitoPostConfirmationFunction", {
    handler: "functions/auth/post_confirmation",
    permissions: ["cognito-idp:AdminAddUserToGroup"],
  });
  const auth = new Cognito(stack, "auth", {
    login: ["email"],
    triggers: {
      postConfirmation,
    },
  });

  new CfnUserPoolGroup(stack, "CustomerGroup", {
    groupName: "Customer",
    description: "Users that signed up via the app. Grants access to non-admin features.",
    userPoolId: auth.userPoolId,
  });

  stack.addOutputs({
    UserPoolId: auth.userPoolId,
    IdentityPoolId: auth.cognitoIdentityPoolId,
    UserPoolClientId: auth.userPoolClientId,
  });

  return { auth };
}
