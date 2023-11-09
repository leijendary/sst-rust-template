package main

import (
	"context"
	"sst-go-template/internal/response"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

func handler(ctx context.Context, event events.APIGatewayV2HTTPRequest) (events.APIGatewayV2HTTPResponse, error) {
	res := response.MappingNotFound()
	return response.ErrorJSON(res)
}

func main() {
	lambda.Start(handler)
}
