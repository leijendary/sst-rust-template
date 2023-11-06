package main

import (
	"context"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider"
)

var groupName = "Customer"

func handler(ctx context.Context, event events.CognitoEventUserPoolsPostConfirmation) (events.CognitoEventUserPoolsPostConfirmation, error) {
	cfg, err := config.LoadDefaultConfig(ctx)
	if err != nil {
		return events.CognitoEventUserPoolsPostConfirmation{}, err
	}

	client := cognitoidentityprovider.NewFromConfig(cfg)
	params := cognitoidentityprovider.AdminAddUserToGroupInput{
		GroupName:  &groupName,
		Username:   &event.UserName,
		UserPoolId: &event.UserPoolID,
	}
	_, err = client.AdminAddUserToGroup(ctx, &params)
	if err != nil {
		return events.CognitoEventUserPoolsPostConfirmation{}, err
	}

	return event, nil
}

func main() {
	lambda.Start(handler)
}
