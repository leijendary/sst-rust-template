package sample

import (
	"context"
	"sst-go-template/internal/db"
)

type Service interface {
	List(ctx context.Context, lang, query string) (*[]Sample, error)
	Create(ctx context.Context, s *Sample) error
	Get(ctx context.Context, id int64) (*Sample, error)
}

type service struct {
	repo *repository
}

func NewService(repo *repository) *service {
	return &service{repo: repo}
}

func (svc *service) Create(ctx context.Context, s *Sample) error {
	tx, err := db.BeginTx(ctx, svc.repo.conn)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	err = svc.repo.save(ctx, tx, s)
	if err != nil {
		return err
	}

	err = svc.repo.saveTranslations(ctx, tx, s.ID, s.Translations)
	if err != nil {
		return err
	}

	return db.Commit(tx)
}

func (svc *service) Get(ctx context.Context, id int64) (*Sample, error) {
	s, err := svc.repo.get(ctx, id)
	if err != nil {
		return nil, err
	}

	ts, err := svc.repo.getTranslations(ctx, id)
	if err != nil {
		return nil, err
	}

	s.Translations = ts

	return s, nil
}
