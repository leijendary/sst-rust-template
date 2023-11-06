package db

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/aws/aws-sdk-go-v2/service/ssm"
	_ "github.com/lib/pq"
)

func Connect(client *ssm.Client) *sql.DB {
	var (
		prefix    = os.Getenv("SST_SSM_PREFIX")
		username  = prefix + "Secret/DB_USERNAME/value"
		password  = prefix + "Secret/DB_PASSWORD/value"
		url       = prefix + "Secret/DB_URL/value"
		name      = prefix + "Secret/DB_NAME/value"
		sslMode   = prefix + "Secret/DB_SSL_MODE/value"
		decrypted = true
		input     = &ssm.GetParametersInput{
			Names:          []string{username, password, url, name, sslMode},
			WithDecryption: &decrypted,
		}
	)
	params, err := client.GetParameters(context.TODO(), input)
	if err != nil {
		panic(err)
	}

	// Replace variable names to their values to be used in the connection string.
	for _, param := range params.Parameters {
		if username == *param.Name {
			username = *param.Value
		}

		if password == *param.Name {
			password = *param.Value
		}

		if url == *param.Name {
			url = *param.Value
		}

		if name == *param.Name {
			name = *param.Value
		}

		if sslMode == *param.Name {
			sslMode = *param.Value
		}
	}

	connStr := fmt.Sprintf("postgres://%s:%s@%s/%s?sslmode=%s", username, password, url, name, sslMode)
	db, err := sql.Open("postgres", connStr)
	if err != nil {
		panic(err)
	}

	return db
}
