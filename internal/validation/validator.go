package validation

import (
	"context"
	"sst-go-template/internal/response"
	"strings"

	"github.com/go-playground/mold/v4/modifiers"
	"github.com/go-playground/validator/v10"
	"github.com/iancoleman/strcase"
)

var (
	conform  = modifiers.New()
	validate = validator.New(validator.WithRequiredStructEnabled())
)

func Validate(v interface{}) error {
	err := conform.Struct(context.Background(), v)
	if err != nil {
		return response.InternalServer()
	}

	err = validate.Struct(v)
	if err == nil {
		return nil
	}

	var (
		status    = 400
		ve        = err.(validator.ValidationErrors)
		errors    = make([]response.Error, len(ve))
		pointer   string
		meta      map[string]any
		namespace []string
		fields    []string
		field     string
	)
	for i, v := range ve {
		meta = map[string]any{}
		namespace = strings.Split(v.Namespace(), ".")[1:]
		fields = []string{}
		for _, ns := range namespace {
			ns = strings.Replace(ns, "[", "/", 1)
			ns = strings.Replace(ns, "]", "", 1)
			fields = append(fields, ns)
		}

		field = strings.Join(fields, "/")
		field = strcase.ToDelimited(field, '/')
		pointer = "/body/" + field

		if len(v.Param()) > 0 {
			if v.Tag() == "unique" {
				status = 409
				pointer += "/" + strcase.ToLowerCamel(v.Param())
			} else {
				meta[v.Tag()] = strcase.ToLowerCamel(v.Param())
			}
		}

		errors[i] = response.Error{
			Code: v.Tag(),
			Source: response.ErrorSource{
				Pointer: pointer,
			},
			Meta: meta,
		}
	}

	return response.ErrorResponse{
		Status: status,
		Errors: errors,
	}
}
