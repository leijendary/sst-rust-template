package adminsample

import (
	"context"
	"sst-go-template/internal/model/sample"
	"sst-go-template/internal/validation"
	"time"
)

type SampleRequest struct {
	Name         string                    `json:"name" mod:"trim" validate:"required,max=100"`
	Description  *string                   `json:"description" mod:"trim"`
	Amount       int32                     `json:"amount" validate:"required,min=1,max=999999999"`
	Version      int8                      `json:"version" validate:"min=0"`
	Translations SampleTranslationRequests `json:"translations" validate:"min=1,unique=Language,unique=Ordinal,dive"`
}

type SampleTranslationRequest struct {
	Name        string  `json:"name" mod:"trim" validate:"required,max=100"`
	Description *string `json:"description" mod:"trim" validate:"max=200"`
	Language    string  `json:"language" mod:"trim" validate:"len=2"`
	Ordinal     int8    `json:"ordinal" validate:"min=1"`
}

type SampleTranslationRequests []*SampleTranslationRequest

type SampleResponse struct {
	ID             int64                        `json:"id"`
	Name           string                       `json:"name"`
	Description    *string                      `json:"description,omitempty"`
	Amount         int32                        `json:"amount"`
	Version        int8                         `json:"version"`
	Translations   []*SampleTranslationResponse `json:"translations,omitempty"`
	CreatedAt      time.Time                    `json:"createdAt"`
	CreatedBy      string                       `json:"createdBy"`
	LastModifiedAt time.Time                    `json:"lastModifiedAt"`
	LastModifiedBy string                       `json:"lastModifiedBy"`
}

type SampleTranslationResponse struct {
	Name        string  `json:"name"`
	Description *string `json:"description,omitempty"`
	Language    string  `json:"language"`
	Ordinal     int8    `json:"ordinal"`
}

func (r *SampleRequest) Validate(ctx context.Context) error {
	// Can return this directly if there are no more validations.
	err := validation.Validate(ctx, r)
	if err != nil {
		return err
	}

	// Enter custom validations here.

	return nil
}

func (st SampleTranslationRequests) ToDatabase() []*sample.SampleTranslation {
	translations := make([]*sample.SampleTranslation, len(st))
	for i, v := range st {
		translation := &sample.SampleTranslation{
			Name:        v.Name,
			Description: v.Description,
			Language:    v.Language,
			Ordinal:     v.Ordinal,
		}
		translations[i] = translation
	}
	return translations
}

func ToListResponse(sa []*sample.Sample) []*SampleResponse {
	list := make([]*SampleResponse, len(sa))
	for i, v := range sa {
		list[i] = &SampleResponse{
			ID:             v.ID,
			Name:           v.Name,
			Description:    v.Description,
			Amount:         v.Amount,
			Version:        v.Version,
			CreatedAt:      v.CreatedAt,
			CreatedBy:      v.CreatedBy,
			LastModifiedAt: v.LastModifiedAt,
			LastModifiedBy: v.LastModifiedBy,
		}
	}
	return list
}

func ToTranslationsResponse(st []*sample.SampleTranslation) []*SampleTranslationResponse {
	translations := make([]*SampleTranslationResponse, len(st))
	for i, v := range st {
		translation := &SampleTranslationResponse{
			Name:        v.Name,
			Description: v.Description,
			Language:    v.Language,
			Ordinal:     v.Ordinal,
		}
		translations[i] = translation
	}
	return translations
}
