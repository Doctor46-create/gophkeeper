package utils

type ctxKey string

const (
	ReqIDKey ctxKey = "request_id"
	UserLoginKey ctxKey = "user_login"  	
	QueryStartTimeKey ctxKey = "query_start_time"
	QuerySQLKey       ctxKey = "query_sql"
	QueryArgsKey      ctxKey = "query_args"
)

