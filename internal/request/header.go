package request

import "golang.org/x/text/language"

func Language(headers map[string]string) language.Tag {
	lang, ok := headers["accept-language"]
	if !ok {
		return language.English
	}

	return language.Make(lang)
}
