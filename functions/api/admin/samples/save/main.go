package main

import (
	"context"
	"encoding/json"
	adminsample "sst-go-template/functions/api/admin/samples"
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
	var req adminsample.SampleRequest
	if err := json.Unmarshal([]byte(event.Body), &req); err != nil {
		return response.InvalidBodyJSON(err)
	}

	if err := req.Validate(ctx); err != nil {
		return response.ErrorJSON(err)
	}

	userId := request.GetUserID(event.RequestContext.Authorizer)
	s := sample.Sample{
		Name:           req.Name,
		Description:    req.Description,
		Amount:         req.Amount,
		Translations:   req.Translations.ToDatabase(),
		CreatedBy:      userId,
		LastModifiedBy: userId,
	}
	if err := service.Create(ctx, &s); err != nil {
		return response.ErrorJSON(err)
	}

	res := &adminsample.SampleResponse{
		ID:             s.ID,
		Name:           s.Name,
		Description:    s.Description,
		Amount:         s.Amount,
		Translations:   adminsample.ToTranslationsResponse(s.Translations),
		CreatedAt:      s.CreatedAt,
		CreatedBy:      s.CreatedBy,
		LastModifiedAt: s.LastModifiedAt,
		LastModifiedBy: s.LastModifiedBy,
	}
	return response.JSON(res, 201)
}

func main() {
	defer conn.Close()

	lambda.Start(handler)
}
