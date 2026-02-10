package config

import (
	"flag"
	"os"
	
	"github.com/ilyakaznacheev/cleanenv"
)

type ServerConfig interface {
	GetRunAddress() string
	GetSecretKey() string
	GetLogLevel() string
	GetDatabaseDSN() string
	GetDBConfig() DBConfig
}

type DBConfig struct {
	MaxConns           int `env:"DB_MAX_CONNS" env-default:"25" yaml:"max_conns"`
	MinConns           int `env:"DB_MIN_CONNS" env-default:"5" yaml:"min_conns"`
	MaxConnLifetimeSec int `env:"DB_MAX_LIFETIME" env-default:"3600" yaml:"max_conn_lifetime"`
}

type serverConfigStruct struct {
	RunAddress  string   `env:"ADDR" env-address:"8080" yaml:"address" env-default:":8080"`
	SecretKey   string   `env:"SECRET_KEY" env-secret-key:"..." yaml:"secret_key" env-default:""`
	LogLevel    string   `env:"LOG_LEVEL" env-log-level:"info" yaml:"log_level" env-default:""`
	DatabaseDSN string   `env:"DATABASE_DSN" env-dsn:"" yaml:"database_dsn" env-default:""`
	DB          DBConfig `env-prefix:"DB_" yaml:"database"`
}

func (c *serverConfigStruct) GetRunAddress() string  { return c.RunAddress }
func (c *serverConfigStruct) GetSecretKey() string   { return c.SecretKey }
func (c *serverConfigStruct) GetLogLevel() string    { return c.LogLevel }
func (c *serverConfigStruct) GetDatabaseDSN() string { return c.DatabaseDSN }
func (c *serverConfigStruct) GetDBConfig() DBConfig  { return c.DB }


func NewServerConfig() ServerConfig {
	cfg := &serverConfigStruct{}

	addrFlag := flag.String("a", "", "address and port to run server")
	secFlag := flag.String("s", "", "secret key for jwt")
	logFlag := flag.String("l", "", "log level")
	dbFlag := flag.String("d", "", "database connection string")
	flag.Parse()

	_, err := os.Stat("configs/server.yaml")
	if err == nil {
		_ = cleanenv.ReadConfig("configs/server.yaml", cfg)
	} else {
		_, err := os.Stat("server.yam")
		if err == nil {
			_ = cleanenv.ReadConfig("server.yaml", cfg)
		}
	}

	if err := cleanenv.ReadEnv(cfg); err != nil {
		panic(err)
	}

	if *addrFlag != "" {
		cfg.RunAddress = *addrFlag
	}
	if *secFlag != "" {
		cfg.SecretKey = *secFlag
	}
	if *logFlag != "" {
		cfg.LogLevel = *logFlag
	}
	if *dbFlag != "" {
		cfg.DatabaseDSN = *dbFlag
	}

	return cfg
}
