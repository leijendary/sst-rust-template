package message

import "fmt"

const DefaultLanguage = "en"

var messages = map[string]map[string]string{
	DefaultLanguage: {
		ValidationRequired:  "%s is a required field",
		ValidationDuplicate: "%s is already taken",
		ResourceNotFound:    "Missing resource. Are you sure this is it?",
		MappingNotFound:     "Where are you going? Are you lost?",
		ServerInternal:      "Oops! Something went wrong",
		RequestInvalid:      "There was something wrong with the request",
	},
}

func Template(lang, key string, args ...any) string {
	language, ok := messages[lang]
	if !ok {
		language = messages[DefaultLanguage]
	}

	msg := language[key]
	if len(args) == 0 {
		return msg
	}

	return fmt.Sprintf(msg, args...)
}
