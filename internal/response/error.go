package response

import (
	"fmt"
)

type ErrorResponse struct {
	Status int     `json:"status"`
	Errors []Error `json:"errors"`
}

type Error struct {
	Id     string         `json:"id,omitempty"`
	Code   string         `json:"code"`
	Source ErrorSource    `json:"source"`
	Meta   map[string]any `json:"meta,omitempty"`
}

type ErrorSource struct {
	Pointer   string `json:"pointer,omitempty"`
	Parameter string `json:"parameter,omitempty"`
	Header    string `json:"header,omitempty"`
}

func (e ErrorResponse) Error() string {
	return fmt.Sprintf("Error %d: %+v", e.Status, e.Errors)
}

func MappingNotFound() ErrorResponse {
	return ErrorResponse{
		Status: 404,
		Errors: []Error{{
			Code:   "not_found",
			Source: ErrorSource{Pointer: "/path"},
		}},
	}
}

func ResourceNotFound(id, pointer string) ErrorResponse {
	return ErrorResponse{
		Status: 404,
		Errors: []Error{{
			Id:     id,
			Code:   "not_found",
			Source: ErrorSource{Pointer: pointer},
		}},
	}
}

func InvalidBody() ErrorResponse {
	return ErrorResponse{
		Status: 400,
		Errors: []Error{{
			Code:   "request_invalid",
			Source: ErrorSource{Pointer: "/body"},
		}},
	}
}

func InternalServer() ErrorResponse {
	return ErrorResponse{
		Status: 500,
		Errors: []Error{{
			Code:   "server_error",
			Source: ErrorSource{Pointer: "/server"},
		}},
	}
}
