package request

import (
	"sst-go-template/internal/response"
	"strconv"
	"time"
)

type Seekable struct {
	Size      int
	CreatedAt *time.Time
	ID        int64
}

type Pageable struct {
	Page   int
	Size   int
	Offset int
}

func GetSeekable(param map[string]string) Seekable {
	size, _ := strconv.Atoi(param["size"])
	if size < 1 {
		size = 20
	}

	var createdAt *time.Time
	ca, err := time.Parse(time.RFC3339Nano, param["createdAt"])
	if err == nil {
		createdAt = &ca
	}

	id, _ := strconv.ParseInt(param["id"], 10, 0)
	return Seekable{
		Size:      size,
		CreatedAt: createdAt,
		ID:        id,
	}
}

func GetPageable(param map[string]string) Pageable {
	page, _ := strconv.Atoi(param["page"])
	if page < 1 {
		page = 1
	}

	size, _ := strconv.Atoi(param["size"])
	if size < 1 {
		size = 20
	}

	offset := (page - 1) * size

	return Pageable{page, size, offset}
}

func GetVersion(param map[string]string) (int8, error) {
	version, err := strconv.ParseInt(param["version"], 10, 0)
	if err != nil {
		return 0, response.ErrorResponse{
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
