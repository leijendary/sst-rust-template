package main

import (
	"context"
	"sst-go-template/internal/request"
	"sst-go-template/internal/response"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

func handler(ctx context.Context, event events.APIGatewayV2HTTPRequest) (events.APIGatewayV2HTTPResponse, error) {
	var (
		lang = request.Language(event.Headers)
		res  = response.MappingNotFound(lang)
	)
	return response.ErrorJSON(res)
}

func main() {
	lambda.Start(handler)
}
