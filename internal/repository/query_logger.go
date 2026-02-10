package repository

import (
	"context"
	"regexp"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/Doctor46-create/gophkeeper/internal/logger"
	"github.com/Doctor46-create/gophkeeper/internal/utils"
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
	ctx = context.WithValue(ctx, utils.QueryStartTimeKey, time.Now())
	ctx = context.WithValue(ctx, utils.QuerySQLKey, data.SQL)
	ctx = context.WithValue(ctx, utils.QueryArgsKey, data.Args)
	
	return ctx
}

func (l *SafeQueryLogger) TraceQueryEnd(ctx context.Context, conn *pgx.Conn, data pgx.TraceQueryEndData) {
	startTime, ok := ctx.Value(utils.QueryStartTimeKey).(time.Time)
	if !ok {
		return
	}
	
	sql, _ := ctx.Value(utils.QuerySQLKey).(string)
	args, _ := ctx.Value(utils.QueryArgsKey).([]any)
	
	duration := time.Since(startTime)

	reqID := "bg-no-req"
	if rid, ok := ctx.Value(utils.ReqIDKey).(string); ok {
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
