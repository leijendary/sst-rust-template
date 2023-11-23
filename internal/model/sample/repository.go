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

	"github.com/lib/pq"
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
	update(ctx context.Context, tx *sql.Tx, id int64, s *Sample) error
	delete(ctx context.Context, u string, id int64, v int8) error
	saveTranslations(ctx context.Context, tx *sql.Tx, id int64, ts []*SampleTranslation) error
	getTranslations(ctx context.Context, id int64) ([]*SampleTranslation, error)
	updateTranslations(ctx context.Context, tx *sql.Tx, id int64, ts []*SampleTranslation) error
}

type repository struct {
	conn *sql.DB
}

func NewRepository(conn *sql.DB) *repository {
	return &repository{conn}
}

func (repo *repository) list(ctx context.Context, q string, p request.Pagination) ([]*Sample, error) {
	const query = `SELECT id, name, description, amount, version, created_at, created_by, last_modified_at, last_modified_by
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
	const query = `SELECT count(*)
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
	const query = `INSERT INTO sample (name, description, amount, created_by, last_modified_by)
	VALUES ($1, $2, $3, $4, $5)
	RETURNING id, name, description, amount, version, created_at, created_by, last_modified_at, last_modified_by`
	row := tx.QueryRowContext(ctx, query, s.Name, s.Description, s.Amount, s.CreatedBy, s.LastModifiedBy)
	if err := row.Scan(&s.ID, &s.Name, &s.Description, &s.Amount, &s.Version, &s.CreatedAt, &s.CreatedBy, &s.LastModifiedAt, &s.LastModifiedBy); err != nil {
		return db.ParseError(err)
	}

	return nil
}

func (repo *repository) get(ctx context.Context, id int64) (*Sample, error) {
	const query = `SELECT id, name, description, amount, version, created_at, created_by, last_modified_at, last_modified_by
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

func (*repository) update(ctx context.Context, tx *sql.Tx, id int64, s *Sample) error {
	const query = `UPDATE sample
	SET
		name = $3,
		description = $4,
		amount = $5,
		version = version + 1,
		last_modified_at = now(),
		last_modified_by = $6
	WHERE id = $1 AND version = $2
	RETURNING id, name, description, amount, version, created_at, created_by, last_modified_at, last_modified_by`
	row := tx.QueryRowContext(ctx, query, id, s.Version, s.Name, s.Description, s.Amount, s.LastModifiedBy)
	if err := row.Scan(&s.ID, &s.Name, &s.Description, &s.Amount, &s.Version, &s.CreatedAt, &s.CreatedBy, &s.LastModifiedAt, &s.LastModifiedBy); err != nil {
		if err == sql.ErrNoRows {
			return response.VersionConflict(id, "/data/sample", s.Version)
		}

		return db.ParseError(err)
	}

	return nil
}

func (repo *repository) delete(ctx context.Context, u string, id int64, v int8) error {
	const query = `UPDATE sample
	SET
		version = version + 1,
		deleted_by = $3,
		deleted_at = now()
	WHERE id = $1 AND version = $2`
	row, err := repo.conn.ExecContext(ctx, query, id, v, u)
	if err != nil {
		return db.ParseError(err)
	}

	count, err := row.RowsAffected()
	if err != nil {
		return db.ParseError(err)
	}

	if count == 0 {
		return response.VersionConflict(id, "/data/sample", v)
	}

	return nil
}

func (*repository) saveTranslations(ctx context.Context, tx *sql.Tx, id int64, ts []*SampleTranslation) error {
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
	const query = `SELECT name, description, language, ordinal
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

func (*repository) updateTranslations(ctx context.Context, tx *sql.Tx, id int64, ts []*SampleTranslation) error {
	query := `DELETE FROM sample_translation WHERE id = $1 AND language NOT IN ($2)`
	langs := make([]string, len(ts))
	for i, v := range ts {
		langs[i] = v.Language
	}

	_, err := tx.ExecContext(ctx, query, id, pq.StringArray(langs))
	if err != nil {
		return db.ParseError(err)
	}

	query = `INSERT INTO sample_translation (id, name, description, language, ordinal)
	VALUES %s
	ON CONFLICT (id, language)
	DO UPDATE SET
		name = EXCLUDED.name,
		description = EXCLUDED.description,
		language = EXCLUDED.language,
		ordinal = EXCLUDED.ordinal
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
