package sample

import (
	"context"
	"go-sst-template/internal/db"
)

type Service interface {
	List(ctx context.Context, lang, query string) (*[]Sample, error)
	Create(ctx context.Context, lang string, s *Sample) error
	Get(ctx context.Context, lang string, id int64) (*Sample, error)
}

type service struct {
	repo *repository
}

func NewService(repo *repository) *service {
	return &service{repo: repo}
}

func (svc *service) Create(ctx context.Context, lang string, s *Sample) error {
	tx, err := db.BeginTx(ctx, svc.repo.conn, lang)
	if err != nil {
		return err
	}

	err = svc.repo.save(ctx, tx, lang, s)
	if err != nil {
		return err
	}

	return db.Commit(tx, lang)
}
