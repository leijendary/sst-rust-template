package sample

import (
	"context"
	"sst-go-template/internal/db"
	"sst-go-template/internal/request"
	"time"
)

type Service interface {
	Seek(ctx context.Context, q string, s request.Seekable) ([]*Sample, *time.Time, int64, error)
	Page(ctx context.Context, q string, p request.Pageable) ([]*Sample, int, error)
	Create(ctx context.Context, s *Sample) error
	Get(ctx context.Context, id int64) (*Sample, error)
	Update(ctx context.Context, id int64, s *Sample) error
	Delete(ctx context.Context, u string, id int64, v int8) error
}

type service struct {
	repo *repository
}

func NewService(repo *repository) *service {
	return &service{repo}
}

func (svc *service) Seek(ctx context.Context, q string, s request.Seekable) ([]*Sample, *time.Time, int64, error) {
	return svc.repo.seek(ctx, q, s)
}

func (svc *service) Page(ctx context.Context, q string, p request.Pageable) ([]*Sample, int, error) {
	list, err := svc.repo.page(ctx, q, p)
	if err != nil {
		return nil, 0, err
	}

	count, err := svc.repo.count(ctx, q)
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

func (svc *service) Delete(ctx context.Context, u string, id int64, v int8) error {
	return svc.repo.delete(ctx, u, id, v)
}
