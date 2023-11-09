import { Cognito, StackContext } from "sst/constructs";

export function CustomerAuthStack({ stack, app }: StackContext) {
  const domainPrefix = `${app.stage === "prod" ? "" : `${app.stage}-`}${app.name}-customer`;
  const auth = new Cognito(stack, "Customer", {
    cdk: {
      userPoolClient: {
        authFlows: {
          userPassword: true,
        },
        generateSecret: true,
        preventUserExistenceErrors: true,
      },
    },
  });
  auth.cdk.userPool.addDomain("CustomerDomain", {
    cognitoDomain: {
      domainPrefix,
    },
  });

  stack.addOutputs({
    CognitoDomain: `https://${domainPrefix}.auth.${stack.region}.amazoncognito.com`,
    UserPoolId: auth.userPoolId,
    IdentityPoolId: auth.cognitoIdentityPoolId,
    UserPoolClientId: auth.userPoolClientId,
  });

  return { auth };
}
