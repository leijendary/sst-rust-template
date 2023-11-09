package db

import (
	"fmt"
	"log"
	"regexp"
	"sst-go-template/internal/response"
	"strings"

	"github.com/iancoleman/strcase"
	"github.com/lib/pq"
)

func ParseError(err error) error {
	e := err.(*pq.Error)
	switch e.Code {
	case "23505":
		return uniqueViolation(e)
	default:
		log.Println(e.Code, e.Message, e.Detail)
		return response.InternalServer()
	}
}

func uniqueViolation(e *pq.Error) error {
	field := e.Column
	if field == "" {
		regex, err := regexp.Compile(`Key \((?:lower\()?([a-zA-Z0-9, ]+)+(?:::text)?\)`)
		if err != nil {
			log.Println(err)
			return response.InternalServer()
		}

		match := regex.FindStringSubmatch(e.Detail)
		field = match[1]
		split := strings.Split(field, ", ")
		field = split[len(split)-1]
	}

	pointer := fmt.Sprintf("/data/%s/%s", strcase.ToLowerCamel(e.Table), strcase.ToLowerCamel(field))
	source := response.ErrorSource{Pointer: pointer}
	return response.ErrorResponse{
		Status: 409,
		Errors: []response.Error{{
			Code:   "duplicate",
			Source: source,
		}},
	}
}
