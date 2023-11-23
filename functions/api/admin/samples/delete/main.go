package main

import (
	"context"
	"sst-go-template/internal/db"
	"sst-go-template/internal/model/sample"
	"sst-go-template/internal/request"
	"sst-go-template/internal/response"
	"sst-go-template/internal/storage"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

var ssmClient = storage.LoadSSMClient()
var conn = db.Connect(ssmClient)
var repo = sample.NewRepository(conn)
var service = sample.NewService(repo)

func handler(ctx context.Context, event events.APIGatewayV2HTTPRequest) (events.APIGatewayV2HTTPResponse, error) {
	id, err := request.GetPathInt64(event.PathParameters, "id")
	if err != nil {
		return response.ErrorJSON(err)
	}

	v, err := request.GetVersion(event.QueryStringParameters)
	if err != nil {
		return response.ErrorJSON(err)
	}

	userId := request.GetUserID(event.RequestContext.Authorizer)
	if err := service.Delete(ctx, userId, id, v); err != nil {
		return response.ErrorJSON(err)
	}

	return response.NoContent()
}

func main() {
	defer conn.Close()

	lambda.Start(handler)
}
