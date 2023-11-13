package request

import (
	"sst-go-template/internal/response"
	"strconv"
)

func PathParamInt64(params map[string]string, key string) (int64, error) {
	param := params[key]
	if len(param) == 0 {
		return -1, response.MappingNotFound()
	}

	value, err := strconv.ParseInt(param, 10, 0)
	if err != nil {
		return -1, response.ErrorResponse{
			Status: 400,
			Errors: []response.Error{{
				Code: "param_invalid",
				Source: response.ErrorSource{
					Parameter: key,
				},
				Meta: map[string]any{
					"type": "int",
				},
			}},
		}
	}

	return value, nil
}
