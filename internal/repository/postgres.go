package repository

import (
	"context"
	"time"

	"github.com/Doctor46-create/gophkeeper/internal/config"
	"github.com/Doctor46-create/gophkeeper/internal/domain"
	"github.com/jackc/pgx/v5"

	"github.com/Doctor46-create/gophkeeper/internal/logger"

	"github.com/jackc/pgx/v5/pgconn"
	"github.com/jackc/pgx/v5/pgxpool"
)

type PostgresStorage struct{ pool *pgxpool.Pool }

func NewPostgresSQLStorage(dsn string, dbCfg config.DBConfig) (*PostgresStorage, error) {
	ctx := context.Background()

	cfg, err := pgxpool.ParseConfig(dsn)
	if err != nil {
		return nil, err
	}

	logger.Log.Infow("Configuring DB Pool",
		"max_conns", dbCfg.MaxConns,
	)

	cfg.MaxConns = int32(dbCfg.MaxConns)
	cfg.MinConns = int32(dbCfg.MinConns)
	cfg.MaxConnLifetime = time.Duration(dbCfg.MaxConnLifetimeSec) * time.Second
	cfg.HealthCheckPeriod = 1 * time.Minute
	cfg.ConnConfig.Tracer = NewSafeQueryLogger(200 * time.Millisecond)

	pool, err := pgxpool.NewWithConfig(ctx, cfg)
	if err != nil {
		return nil, err
	}

	if err := pool.Ping(ctx); err != nil {
		pool.Close()
		return nil, err
	}

	logger.Log.Info("PostgreSQL connected")

	return &PostgresStorage{pool: pool}, nil
}

func (s *PostgresStorage) Close() error {
	s.pool.Close()
	return nil
}

func (s *PostgresStorage) exec(
	ctx context.Context,
	sql string,
	args ...any,
) (pgconn.CommandTag, error) {
	if tx, ok := ctx.Value(txKey).(pgx.Tx); ok {
		return tx.Exec(ctx, sql, args...)
	}

	return s.pool.Exec(ctx, sql, args...)
}

func (s *PostgresStorage) RunInTransaction(
	ctx context.Context,
	fn func(ctx context.Context) error,
) error {
	tx, err := s.pool.Begin(ctx)
	if err != nil {
		return err
	}

	txCtx := context.WithValue(ctx, txKey, tx)
	defer tx.Rollback(ctx)

	if err := fn(txCtx); err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (s *PostgresStorage) CreateUser(
	ctx context.Context,
	login, hash string,
) error {
	_, err := s.exec(ctx, userCreate, login, hash)
	return HandleDBError(err)
}

func (s *PostgresStorage) GetUser(
	ctx context.Context,
	login string,
) (string, error) {
	var hash string
	err := s.pool.QueryRow(ctx, userGet, login).Scan(&hash)
	if err != nil {
		return "", HandleDBError(err)
	}

	return hash, nil
}

func (s *PostgresStorage) SaveSecrets(
	ctx context.Context,
	login string,
	secrets []domain.Secret,
) error {
	if len(secrets) == 0 {
		return nil
	}

	return s.RunInTransaction(ctx, func(txCtx context.Context) error {
		batch := &pgx.Batch{}

		for _, sec := range secrets {
			createdAt := sec.CreatedAt
			if createdAt.IsZero() {
				createdAt = time.Now()
			}

			updatedAt := sec.UpdatedAt
			if updatedAt.IsZero() {
				updatedAt = time.Now()
			}

			batch.Queue(
				secretUpsert,
				sec.ID,
				login,
				sec.Type,
				sec.Data,
				createdAt,
				updatedAt,
			)
		}

		br := s.pool.SendBatch(txCtx, batch)
		defer br.Close()

		for i := 0; i < batch.Len(); i++ {
			if _, err := br.Exec(); err != nil {
				return err
			}
		}

		return nil
	})
}

func (s *PostgresStorage) AddData(
	ctx context.Context,
	login string,
	secret domain.Secret,
) error {
	return s.SaveSecrets(ctx, login, []domain.Secret{secret})
}

func (s *PostgresStorage) GetData(
	ctx context.Context,
	login string,
) ([]domain.Secret, error) {
	rows, err := s.pool.Query(ctx, secretGet, login)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return pgx.CollectRows(rows, func(row pgx.CollectableRow) (domain.Secret, error) {
		var s domain.Secret
		err := row.Scan(
			&s.ID,
			&s.UserLogin,
			&s.Type,
			&s.Data,
			&s.CreatedAt,
			&s.UpdatedAt,
		)
		return s, err
	})
}

func (s *PostgresStorage) DeleteData(
	ctx context.Context,
	login, id string,
) error {
	res, err := s.exec(ctx, secretDelete, id, login)
	if err != nil {
		return err
	}

	if res.RowsAffected() == 0 {
		return domain.ErrSecretNotFound
	}

	return nil
}
