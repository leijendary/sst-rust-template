package model

import "time"

type Seek[T any] struct {
	Data      []*T       `json:"data"`
	Size      int        `json:"size"`
	CreatedAt *time.Time `json:"createdAt,omitempty"`
	ID        int64      `json:"id"`
}

type Page[T any] struct {
	Data  []*T `json:"data"`
	Page  int  `json:"page"`
	Size  int  `json:"size"`
	Total int  `json:"total"`
}
