package adminsample

import (
	"sst-go-template/internal/model/sample"
	"sst-go-template/internal/validation"
	"time"
)

type SampleRequest struct {
	Name         string                    `json:"name" mod:"trim" validate:"required,max=100"`
	Description  string                    `json:"description" mod:"trim"`
	Amount       float64                   `json:"amount" validate:"required,min=0.01,max=9999999999.99"`
	Translations SampleTranslationRequests `json:"translations" validate:"min=1,unique=Language,unique=Ordinal,dive"`
}

type SampleTranslationRequest struct {
	Name        string `json:"name" mod:"trim" validate:"required,max=100"`
	Description string `json:"description" mod:"trim" validate:"max=200"`
	Language    string `json:"language" mod:"trim" validate:"len=2"`
	Ordinal     int8   `json:"ordinal" validate:"min=1"`
}

type SampleTranslationRequests []*SampleTranslationRequest

type SampleResponse struct {
	ID           int64                        `json:"id"`
	Name         string                       `json:"name"`
	Description  string                       `json:"description,omitempty"`
	Amount       float64                      `json:"amount"`
	Version      int8                         `json:"version"`
	Translations []*SampleTranslationResponse `json:"translations"`
	CreatedAt    time.Time                    `json:"createdAt"`
}

type SampleTranslationResponse struct {
	Name        string `json:"name"`
	Description string `json:"description,omitempty"`
	Language    string `json:"language"`
	Ordinal     int8   `json:"ordinal"`
}

func (r *SampleRequest) Validate() error {
	// Can return this directly if there are no more validations.
	err := validation.Validate(r)
	if err != nil {
		return err
	}

	// Enter custom validations here.

	return nil
}

func (st SampleTranslationRequests) ToDatabase() []*sample.SampleTranslation {
	translations := make([]*sample.SampleTranslation, len(st))
	var translation *sample.SampleTranslation
	for i, v := range st {
		translation = &sample.SampleTranslation{
			Name:        v.Name,
			Description: v.Description,
			Language:    v.Language,
			Ordinal:     v.Ordinal,
		}
		translations[i] = translation
	}

	return translations
}

func ToTranslationsResponse(st []*sample.SampleTranslation) []*SampleTranslationResponse {
	translations := make([]*SampleTranslationResponse, len(st))
	var translation *SampleTranslationResponse
	for i, v := range st {
		translation = &SampleTranslationResponse{
			Name:        v.Name,
			Description: v.Description,
			Language:    v.Language,
			Ordinal:     v.Ordinal,
		}
		translations[i] = translation
	}

	return translations
}
