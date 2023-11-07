package response

import (
	"fmt"
	"sst-go-template/internal/message"
)

type ErrorResponse struct {
	Status int     `json:"status"`
	Errors []Error `json:"errors"`
}

type Error struct {
	Code    string      `json:"code"`
	Message string      `json:"message,omitempty"`
	Source  ErrorSource `json:"source"`
}

type ErrorSource struct {
	Pointer   string `json:"pointer,omitempty"`
	Parameter string `json:"parameter,omitempty"`
	Header    string `json:"header,omitempty"`
}

func (e ErrorResponse) Error() string {
	return fmt.Sprintf("Error %d: %+v", e.Status, e.Errors)
}

func BuildError(lang, code string, source ErrorSource, args ...any) Error {
	return Error{
		Code:    code,
		Message: message.Template(lang, code, args...),
		Source:  source,
	}
}

func InternalServer(lang string) ErrorResponse {
	return ErrorResponse{
		Status: 500,
		Errors: []Error{
			BuildError(lang, message.ServerInternal, ErrorSource{Pointer: "/server/internal"}),
		},
	}
}

func InvalidBody(lang string) ErrorResponse {
	return ErrorResponse{
		Status: 400,
		Errors: []Error{
			BuildError(lang, message.RequestInvalid, ErrorSource{Pointer: "/body"}),
		},
	}
}
