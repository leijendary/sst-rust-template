package request

import "go-sst-template/internal/message"

func Language(headers map[string]string) string {
	lang, ok := headers["accept-language"]
	if !ok {
		return message.DefaultLanguage
	}

	return lang
}
