package main

import (
	"context"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

func handler(ctx context.Context, event events.APIGatewayV2HTTPRequest) (events.APIGatewayV2HTTPResponse, error) {
	println(event.Body)

	return events.APIGatewayV2HTTPResponse{
		Body: event.Body,
	}, nil
}

func main() {
	lambda.Start(handler)
}
