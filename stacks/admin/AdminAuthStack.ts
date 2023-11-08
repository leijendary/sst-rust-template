import { Cognito, StackContext } from "sst/constructs";

export function AdminAuthStack({ stack, app }: StackContext) {
  const domainPrefix = `${app.name}-admin${stack.stage === "prod" ? "" : `-${stack.stage}`}`;
  const auth = new Cognito(stack, "Admin", {
    cdk: {
      userPool: {
        selfSignUpEnabled: false,
      },
      userPoolClient: {
        authFlows: {
          adminUserPassword: true,
          userPassword: true,
        },
        preventUserExistenceErrors: true,
      },
    },
    login: ["email"],
  });
  auth.cdk.userPool.addDomain("AdminDomain", {
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
