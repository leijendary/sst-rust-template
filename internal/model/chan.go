package model

type ResultErr[T any] struct {
	Result T
	Err    error
}
