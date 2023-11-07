package main

import (
	"context"
	"encoding/json"
	samplev1 "sst-go-template/functions/v1/samples"
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
	var (
		lang = request.Language(event.Headers)
		req  = samplev1.SampleRequest{}
	)
	if err := json.Unmarshal([]byte(event.Body), &req); err != nil {
		return response.InvalidBodyJSON(lang, err)
	}

	if err := req.Validate(lang); err != nil {
		return response.ErrorJSON(err)
	}

	s := sample.Sample{
		Name:           req.Name,
		Description:    req.Description,
		Amount:         req.Amount,
		Translations:   req.Translations,
		CreatedBy:      "System",
		LastModifiedBy: "System",
	}
	if err := service.Create(ctx, lang, &s); err != nil {
		return response.ErrorJSON(err)
	}

	res := &samplev1.SampleResponse{
		ID:           s.ID,
		Name:         s.Name,
		Description:  s.Description,
		Amount:       s.Amount,
		Translations: s.Translations,
		CreatedAt:    s.CreatedAt,
	}
	return response.JSON(res, 200)
}

func main() {
	defer conn.Close()

	lambda.Start(handler)
}
