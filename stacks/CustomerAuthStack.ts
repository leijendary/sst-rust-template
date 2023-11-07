import { Cognito, StackContext } from "sst/constructs";

export default function CustomerAuthStack({ stack }: StackContext) {
  const auth = new Cognito(stack, "customer");

  stack.addOutputs({
    UserPoolId: auth.userPoolId,
    IdentityPoolId: auth.cognitoIdentityPoolId,
    UserPoolClientId: auth.userPoolClientId,
  });

  return { auth };
}