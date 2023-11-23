package main

import (
	"context"
	adminsample "sst-go-template/functions/api/admin/samples"
	"sst-go-template/internal/db"
	"sst-go-template/internal/model"
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
	var (
		params = event.QueryStringParameters
		q      = params["query"]
		p      = request.GetPagination(params)
	)
	list, total, err := service.List(ctx, q, p)
	if err != nil {
		return response.ErrorJSON(err)
	}

	res := model.Page[adminsample.SampleResponse]{
		Data:  adminsample.ToListResponse(list),
		Page:  p.Page,
		Size:  p.Size,
		Total: total,
	}
	return response.JSON(res, 200)
}

func main() {
	lambda.Start(handler)
}
