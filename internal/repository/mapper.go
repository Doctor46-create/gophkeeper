package repository

import (
	"errors"

	"github.com/Doctor46-create/gophkeeper/internal/domain"
	"github.com/jackc/pgx/v5/pgconn"
)

func HandleDBError(err error) error {
	if err == nil {
		return nil
	}

	var pgErr *pgconn.PgError
	if errors.As(err, &pgErr) {
		switch pgErr.Code {
		case "23505":
			return domain.ErrUserExists
		case "23503":
			return domain.ErrSecretNotFound
		case "40001":
			return errors.New("serialization conflict")
		}
	}
	return err
}
