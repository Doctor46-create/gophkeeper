package config

type ServerConfig interface {
	GetRunAddress() string
	GetSecretKey() string
	GetLogLevel() string
	GetDatabaseDSN() string
	GetDBConfig() DBConfig
}
