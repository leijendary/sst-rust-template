package main

import (
	"go-sst-template/internal/response"
	"log"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

func handler(event events.APIGatewayV2HTTPRequest) (events.APIGatewayV2HTTPResponse, error) {
	log.Println(event.RequestContext.Authorizer.JWT.Claims)
	log.Println(event.RequestContext.Authorizer.JWT.Scopes)
	log.Println(event.RequestContext.Authorizer.IAM.CognitoIdentity.IdentityPoolID)
	log.Println(event.RequestContext.Authorizer.IAM.UserID)
	return response.JSON(nil, 200)
}

func main() {
	lambda.Start(handler)
}
