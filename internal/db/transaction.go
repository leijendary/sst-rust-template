package db

import (
	"context"
	"database/sql"
)

func BeginTx(ctx context.Context, conn *sql.DB, lang string) (*sql.Tx, error) {
	tx, err := conn.BeginTx(ctx, &sql.TxOptions{})
	if err != nil {
		return nil, ParseError(lang, err)
	}

	return tx, nil
}

func Commit(tx *sql.Tx, lang string) error {
	err := tx.Commit()
	if err != nil {
		return ParseError(lang, err)
	}

	return nil
}
