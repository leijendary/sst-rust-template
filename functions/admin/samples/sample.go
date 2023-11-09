package adminsample

import (
	"sst-go-template/internal/model/sample"
	"sst-go-template/internal/validation"
	"time"
)

type SampleRequest struct {
	Name         string                      `json:"name" mod:"trim" validate:"required,max=100"`
	Description  string                      `json:"description" mod:"trim"`
	Amount       float64                     `json:"amount" validate:"required,min=0.01,max=9999999999.99"`
	Translations []*sample.SampleTranslation `json:"translations" validate:"min=1,unique=Language,unique=Ordinal,dive"`
}

type SampleResponse struct {
	ID           int64                       `json:"id"`
	Name         string                      `json:"name"`
	Description  string                      `json:"description,omitempty"`
	Amount       float64                     `json:"amount"`
	Translations []*sample.SampleTranslation `json:"translations"`
	CreatedAt    time.Time                   `json:"createdAt"`
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
