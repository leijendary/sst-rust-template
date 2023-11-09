package request

import (
	"sst-go-template/internal/response"
	"strconv"
)

func PathParamInt64(params map[string]string, k string) (int64, error) {
	p := params[k]
	if len(p) == 0 {
		return -1, response.MappingNotFound()
	}

	i, err := strconv.ParseInt(p, 10, 0)
	if err != nil {
		return -1, response.ErrorResponse{
			Status: 400,
			Errors: []response.Error{{
				Code: "param_invalid",
				Source: response.ErrorSource{
					Parameter: k,
				},
			}},
		}
	}

	return i, nil
}
