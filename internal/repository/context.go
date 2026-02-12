package repository

type ctxKey string

const (
	txKey             ctxKey = "tx"
	ReqIDKey          ctxKey = "request_id"
	QueryStartTimeKey ctxKey = "query_start_time"
	QuerySQLKey       ctxKey = "query_sql"
	QueryArgsKey      ctxKey = "query_args"
)
