package response

import (
	"encoding/json"
	"log"

	"github.com/aws/aws-lambda-go/events"
)

var headers = map[string]string{"content-type": "application/json"}

func JSON(v any, status int) (events.APIGatewayV2HTTPResponse, error) {
	b, err := json.Marshal(v)
	if err != nil {
		return events.APIGatewayV2HTTPResponse{}, err
	}

	return events.APIGatewayV2HTTPResponse{
		Body:       string(b),
		StatusCode: status,
		Headers:    headers,
	}, nil
}

func NoContent() (events.APIGatewayV2HTTPResponse, error) {
	return events.APIGatewayV2HTTPResponse{
		Body:       "",
		StatusCode: 204,
		Headers:    headers,
	}, nil
}

func ErrorJSON(err error) (events.APIGatewayV2HTTPResponse, error) {
	e := err.(ErrorResponse)
	return JSON(e, e.Status)
}

func InvalidBodyJSON(err error) (events.APIGatewayV2HTTPResponse, error) {
	log.Println(err)
	res := InvalidBody()
	return JSON(res, res.Status)
}

func ServerErrorJSON(err error) (events.APIGatewayV2HTTPResponse, error) {
	log.Println(err)
	res := InternalServer()
	return JSON(res, res.Status)
}
