package sample

import (
	"context"
	"database/sql"
	"fmt"
	"go-sst-template/internal/db"
	"strings"
	"time"
)

type Sample struct {
	ID             int64               `json:"id"`
	Name           string              `json:"name"`
	Description    string              `json:"description"`
	Amount         float32             `json:"amount"`
	Translations   []SampleTranslation `json:"translations"`
	CreatedAt      time.Time           `json:"createdAt"`
	CreatedBy      string              `json:"createdBy"`
	LastModifiedAt time.Time           `json:"lastModifiedAt"`
	LastModifiedBy string              `json:"lastModifiedBy"`
}

type SampleTranslation struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	Language    string `json:"language"`
	Ordinal     int8   `json:"ordinal"`
}

type Repository interface {
	save(ctx context.Context, tx *sql.Tx, lang string, s *Sample) error
}

type repository struct {
	conn *sql.DB
}

func NewRepository(conn *sql.DB) *repository {
	return &repository{conn: conn}
}

func (r *repository) save(ctx context.Context, tx *sql.Tx, lang string, s *Sample) error {
	query := `INSERT INTO sample (name, description, amount, created_by, last_modified_by)
	VALUES ($1, $2, $3, $4, $5)
	RETURNING id, name, description, amount, created_at`
	row := tx.QueryRowContext(ctx, query, s.Name, s.Description, s.Amount, s.CreatedBy, s.LastModifiedBy)
	if err := row.Scan(&s.ID, &s.Name, &s.Description, &s.Amount, &s.CreatedAt); err != nil {
		return db.ParseError(lang, err)
	}

	if err := saveTranslations(ctx, tx, lang, s.ID, s.Translations); err != nil {
		return err
	}

	return nil
}

func saveTranslations(ctx context.Context, tx *sql.Tx, lang string, id int64, st []SampleTranslation) error {
	if len(st) == 0 {
		return nil
	}

	query := `INSERT INTO sample_translation (id, name, description, language, ordinal)
	VALUES %s
	RETURNING name, description, language, ordinal`
	params := []string{}
	args := []any{}
	for i, v := range st {
		param := fmt.Sprintf("($%d, $%d, $%d, $%d, $%d)", i*5+1, i*5+2, i*5+3, i*5+4, i*5+5)
		params = append(params, param)
		args = append(args, id, v.Name, v.Description, v.Language, v.Ordinal)
	}

	param := strings.Join(params, ", ")
	query = fmt.Sprintf(query, param)
	rows, err := tx.QueryContext(ctx, query, args...)
	if err != nil {
		return db.ParseError(lang, err)
	}
	defer rows.Close()

	for i := 0; rows.Next(); i++ {
		if err := rows.Scan(&st[i].Name, &st[i].Description, &st[i].Language, &st[i].Ordinal); err != nil {
			return db.ParseError(lang, err)
		}
	}

	return nil
}
