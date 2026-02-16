package repository

import (
	"context"
	"regexp"
	"time"

	"github.com/Doctor46-create/gophkeeper/internal/logger"
	"github.com/jackc/pgx/v5"
)

var (
	tableExtractor  = regexp.MustCompile(`(?im)(?:from|into|update)\s+(\w+)`)
	sensitiveTables = map[string]bool{"secrets": true, "users": true}
)

type SafeQueryLogger struct {
	SlowThreshold time.Duration
}

func NewSafeQueryLogger(slowThreshold time.Duration) *SafeQueryLogger {
	return &SafeQueryLogger{SlowThreshold: slowThreshold}
}

func (l *SafeQueryLogger) TraceQueryStart(
	ctx context.Context,
	conn *pgx.Conn,
	data pgx.TraceQueryStartData,
) context.Context {
	ctx = context.WithValue(ctx, QueryStartTimeKey, time.Now())
	ctx = context.WithValue(ctx, QuerySQLKey, data.SQL)
	ctx = context.WithValue(ctx, QueryArgsKey, data.Args)

	return ctx
}

func (l *SafeQueryLogger) TraceQueryEnd(ctx context.Context, conn *pgx.Conn, data pgx.TraceQueryEndData) {
	startTime, ok := ctx.Value(QueryStartTimeKey).(time.Time)
	if !ok {
		return
	}

	sql, _ := ctx.Value(QuerySQLKey).(string)
	args, _ := ctx.Value(QueryArgsKey).([]any)

	duration := time.Since(startTime)

	reqID := "bg-no-req"
	if rid, ok := ctx.Value(ReqIDKey).(string); ok {
		reqID = rid
	}

	if duration > l.SlowThreshold {
		logger.Log.Warnw("Slow SQL",
			"request_id", reqID,
			"sql", sql,
			"duration", duration,
			"args", args,
		)
	} else {
		logger.Log.Debugw("SQL",
			"request_id", reqID,
			"sql", sql,
			"duration", duration,
			"args", args,
		)
	}
}
