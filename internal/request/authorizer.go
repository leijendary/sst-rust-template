package request

import (
	"github.com/aws/aws-lambda-go/events"
)

func UserID(auth *events.APIGatewayV2HTTPRequestContextAuthorizerDescription) string {
	return auth.JWT.Claims["sub"]
}
