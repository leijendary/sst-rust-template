import { Cognito, StackContext } from "sst/constructs";

export function AdminAuth({ stack, app }: StackContext) {
  const domainPrefix = `${app.stage === "prod" ? "" : `${app.stage}-`}${app.name}-admin`;
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
        generateSecret: true,
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
