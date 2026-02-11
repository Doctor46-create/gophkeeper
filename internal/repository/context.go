package repository

type ctxKey string

const (
	txKey             ctxKey = "tx"
	QueryStartTimeKey ctxKey = "query_start_time"
	QuerySQLKey       ctxKey = "query_sql"
	QueryArgsKey      ctxKey = "query_args"
	ReqIDKey          ctxKey = "request_id"
)
