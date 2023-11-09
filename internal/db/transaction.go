package db

import (
	"context"
	"database/sql"
)

func BeginTx(ctx context.Context, conn *sql.DB) (*sql.Tx, error) {
	tx, err := conn.BeginTx(ctx, &sql.TxOptions{})
	if err != nil {
		return nil, ParseError(err)
	}

	return tx, nil
}

func Commit(tx *sql.Tx) error {
	err := tx.Commit()
	if err != nil {
		return ParseError(err)
	}

	return nil
}
