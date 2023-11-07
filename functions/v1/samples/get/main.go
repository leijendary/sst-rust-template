package main

import (
	"log"
	"sst-go-template/internal/response"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

func handler(event events.APIGatewayV2HTTPRequest) (events.APIGatewayV2HTTPResponse, error) {
	log.Println(event.RequestContext.Authorizer.IAM.CognitoIdentity.IdentityID)
	log.Println(event.RequestContext.Authorizer.IAM.UserID)
	return response.JSON(nil, 200)
}

func main() {
	lambda.Start(handler)
}
