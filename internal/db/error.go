package db

import (
	"fmt"
	"go-sst-template/internal/message"
	"go-sst-template/internal/response"
	"log"
	"regexp"
	"strings"

	"github.com/iancoleman/strcase"
	"github.com/lib/pq"
)

func ParseError(lang string, err error) error {
	e := err.(*pq.Error)
	switch e.Code {
	case "23505":
		return uniqueViolation(lang, e)
	default:
		log.Println(e.Code, e.Message)
		return response.InternalServer(lang)
	}
}

func uniqueViolation(lang string, e *pq.Error) error {
	field := e.Column
	if field == "" {
		regex, err := regexp.Compile(`Key \((?:lower\()?([a-zA-Z0-9, ]+)+(?:::text)?\)`)
		if err != nil {
			log.Println(err)
			return response.InternalServer(lang)
		}

		match := regex.FindStringSubmatch(e.Detail)
		field = match[1]
		split := strings.Split(field, ", ")
		field = split[len(split)-1]
	}

	field = strings.ToUpper(field[:1]) + field[1:]
	pointer := fmt.Sprintf("/data/%s/%s", strcase.ToCamel(e.Table), strcase.ToLowerCamel(field))
	source := response.ErrorSource{Pointer: pointer}
	return response.ErrorResponse{
		Status: 409,
		Errors: []response.Error{
			response.BuildError(lang, message.ValidationDuplicate, source, field),
		},
	}
}
