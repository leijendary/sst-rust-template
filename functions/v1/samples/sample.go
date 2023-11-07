package samplev1

import (
	"sst-go-template/internal/model/sample"
	"time"
)

type SampleRequest struct {
	Name         string                     `json:"name"`
	Description  string                     `json:"description"`
	Amount       float32                    `json:"amount"`
	Translations []sample.SampleTranslation `json:"translations"`
}

type SampleResponse struct {
	ID           int64                      `json:"id"`
	Name         string                     `json:"name"`
	Description  string                     `json:"description"`
	Amount       float32                    `json:"amount"`
	Translations []sample.SampleTranslation `json:"translations"`
	CreatedAt    time.Time                  `json:"createdAt"`
}

func (r *SampleRequest) Validate(lang string) error {
	return nil
}
