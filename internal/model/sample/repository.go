package sample

import (
	"context"
	"database/sql"
	"fmt"
	"sst-go-template/internal/db"
	"sst-go-template/internal/request"
	"sst-go-template/internal/response"
	"strings"
	"time"
)

type Sample struct {
	ID             int64
	Name           string
	Description    *string
	Amount         int32
	Version        int8
	Translations   []*SampleTranslation
	CreatedAt      time.Time
	CreatedBy      string
	LastModifiedAt time.Time
	LastModifiedBy string
}

type SampleTranslation struct {
	Name        string
	Description *string
	Language    string
	Ordinal     int8
}

type Repository interface {
	list(ctx context.Context, q string, p request.Pagination) ([]*Sample, error)
	count(ctx context.Context, q string) (int, error)
	save(ctx context.Context, tx *sql.Tx, s *Sample) error
	get(ctx context.Context, id int64) (*Sample, error)
	saveTranslations(ctx context.Context, tx *sql.Tx, id int64, ts []*SampleTranslation) error
	getTranslations(ctx context.Context, id int64) ([]*SampleTranslation, error)
}

type repository struct {
	conn *sql.DB
}

func NewRepository(conn *sql.DB) *repository {
	return &repository{conn}
}

func (repo *repository) list(ctx context.Context, q string, p request.Pagination) ([]*Sample, error) {
	query := `SELECT id, name, description, amount, version, created_at, created_by, last_modified_at, last_modified_by
	FROM sample
	WHERE name ILIKE CONCAT('%%', $1::text, '%%') AND deleted_at IS NULL
	LIMIT $2
	OFFSET $3`
	rows, err := repo.conn.QueryContext(ctx, query, q, p.Size, p.Offset)
	if err != nil {
		return nil, db.ParseError(err)
	}
	defer rows.Close()

	var ss []*Sample
	for i := 0; rows.Next(); i++ {
		var s Sample
		if err := rows.Scan(&s.ID, &s.Name, &s.Description, &s.Amount, &s.Version, &s.CreatedAt, &s.CreatedBy, &s.LastModifiedAt, &s.LastModifiedBy); err != nil {
			return nil, db.ParseError(err)
		}
		ss = append(ss, &s)
	}
	return ss, nil
}

func (repo *repository) count(ctx context.Context, q string) (int, error) {
	query := `SELECT count(*)
	FROM sample
	WHERE name ILIKE CONCAT('%%', $1::text, '%%') AND deleted_at IS NULL`
	row := repo.conn.QueryRowContext(ctx, query, q)
	var count int
	if err := row.Scan(&count); err != nil {
		return 0, db.ParseError(err)
	}
	return count, nil
}

func (*repository) save(ctx context.Context, tx *sql.Tx, s *Sample) error {
	query := `INSERT INTO sample (name, description, amount, created_by, last_modified_by)
	VALUES ($1, $2, $3, $4, $5)
	RETURNING id, name, description, amount, version, created_at, created_by, last_modified_at, last_modified_by`
	row := tx.QueryRowContext(ctx, query, s.Name, s.Description, s.Amount, s.CreatedBy, s.LastModifiedBy)
	if err := row.Scan(&s.ID, &s.Name, &s.Description, &s.Amount, &s.Version, &s.CreatedAt, &s.CreatedBy, &s.LastModifiedAt, &s.LastModifiedBy); err != nil {
		return db.ParseError(err)
	}

	return nil
}

func (repo *repository) get(ctx context.Context, id int64) (*Sample, error) {
	query := `SELECT id, name, description, amount, version, created_at, created_by, last_modified_at, last_modified_by
	FROM sample
	WHERE id = $1 AND deleted_at IS NULL`
	row := repo.conn.QueryRowContext(ctx, query, id)
	var s Sample
	if err := row.Scan(&s.ID, &s.Name, &s.Description, &s.Amount, &s.Version, &s.CreatedAt, &s.CreatedBy, &s.LastModifiedAt, &s.LastModifiedBy); err != nil {
		if err == sql.ErrNoRows {
			return nil, response.ResourceNotFound(id, "/data/sample")
		}

		return nil, db.ParseError(err)
	}
	return &s, nil
}

func (*repository) saveTranslations(ctx context.Context, tx *sql.Tx, id int64, ts []*SampleTranslation) error {
	if len(ts) == 0 {
		return nil
	}

	query := `INSERT INTO sample_translation (id, name, description, language, ordinal)
	VALUES %s
	RETURNING name, description, language, ordinal`
	var params []string
	var args []any
	for i, v := range ts {
		param := fmt.Sprintf("($%d, $%d, $%d, $%d, $%d)", i*5+1, i*5+2, i*5+3, i*5+4, i*5+5)
		params = append(params, param)
		args = append(args, id, v.Name, v.Description, v.Language, v.Ordinal)
	}

	param := strings.Join(params, ", ")
	query = fmt.Sprintf(query, param)
	rows, err := tx.QueryContext(ctx, query, args...)
	if err != nil {
		return db.ParseError(err)
	}
	defer rows.Close()

	for i := 0; rows.Next(); i++ {
		if err := rows.Scan(&ts[i].Name, &ts[i].Description, &ts[i].Language, &ts[i].Ordinal); err != nil {
			return db.ParseError(err)
		}
	}

	return nil
}

func (repo *repository) getTranslations(ctx context.Context, id int64) ([]*SampleTranslation, error) {
	query := `SELECT name, description, language, ordinal
	FROM sample_translation
	WHERE id = $1`
	rows, err := repo.conn.QueryContext(ctx, query, id)
	if err != nil {
		return nil, db.ParseError(err)
	}
	defer rows.Close()

	var ts []*SampleTranslation
	for rows.Next() {
		var t SampleTranslation
		if err := rows.Scan(&t.Name, &t.Description, &t.Language, &t.Ordinal); err != nil {
			return nil, db.ParseError(err)
		}
		ts = append(ts, &t)
	}
	return ts, nil
}
