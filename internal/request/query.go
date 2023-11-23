package request

import (
	"sst-go-template/internal/response"
	"strconv"
)

type Pagination struct {
	Page   int
	Size   int
	Offset int
}

func GetPagination(param map[string]string) Pagination {
	page, _ := strconv.Atoi(param["page"])
	if page < 1 {
		page = 1
	}

	size, _ := strconv.Atoi(param["size"])
	if size < 1 {
		size = 20
	}

	offset := (page - 1) * size

	return Pagination{page, size, offset}
}

func GetVersion(param map[string]string) (int8, error) {
	version, err := strconv.ParseInt(param["version"], 10, 8)
	if err != nil {
		return -1, response.ErrorResponse{
			Status: 400,
			Errors: []response.Error{{
				Code:   "param_invalid",
				Source: response.ErrorSource{Parameter: "version"},
				Meta:   map[string]any{"type": "int"},
			}},
		}
	}
	return int8(version), nil
}
