package sample

import (
	"context"
	"sst-go-template/internal/db"
	"sst-go-template/internal/request"
)

type Service interface {
	List(ctx context.Context, q string, p request.Pagination) ([]*Sample, int, error)
	Create(ctx context.Context, s *Sample) error
	Get(ctx context.Context, id int64) (*Sample, error)
	Update(ctx context.Context, id int64, s *Sample) error
}

type service struct {
	repo *repository
}

func NewService(repo *repository) *service {
	return &service{repo}
}

func (svc *service) List(ctx context.Context, q string, p request.Pagination) ([]*Sample, int, error) {
	list, err := svc.repo.list(context.Background(), q, p)
	if err != nil {
		return nil, 0, err
	}

	count, err := svc.repo.count(context.Background(), q)
	if err != nil {
		return nil, 0, err
	}

	return list, count, nil
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

func (svc *service) Update(ctx context.Context, id int64, s *Sample) error {
	tx, err := db.BeginTx(ctx, svc.repo.conn)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	err = svc.repo.update(ctx, tx, id, s)
	if err != nil {
		return err
	}

	err = svc.repo.updateTranslations(ctx, tx, s.ID, s.Translations)
	if err != nil {
		return err
	}

	return db.Commit(tx)
}
