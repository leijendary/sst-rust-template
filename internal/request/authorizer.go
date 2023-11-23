package request

import (
	"github.com/aws/aws-lambda-go/events"
)

func GetUserID(auth *events.APIGatewayV2HTTPRequestContextAuthorizerDescription) string {
	return auth.JWT.Claims["sub"]
}
