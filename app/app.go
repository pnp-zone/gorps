package app

import (
	"github.com/labstack/echo/v4"
	"github.com/myOmikron/echotools/database"
	"github.com/myOmikron/echotools/execution"
	"github.com/myOmikron/echotools/logging"
	"github.com/myOmikron/echotools/middleware"
	"github.com/pnp-zone/gorps/handler"
	"github.com/pnp-zone/gorps/models"
	"gorm.io/driver/sqlite"
)

func StartServer(configPath string) {
	logging.Initialize(&logging.Config{})
	log := logging.GetLogger("app")

	config := ParseConfig(configPath)

	e := echo.New()

	driver := sqlite.Open("gorps-server.sqlite3")
	db := database.Initialize(driver,
		&models.Advantage{},
		&models.Disadvantage{},
		&models.Perk{},
		&models.Quirk{},
		&models.Modifier{},
		&models.Feature{},
		&models.PreReqs{},
		&models.Skill{},
		&models.Character{},
		&models.Default{},
	)

	e.Use(middleware.CustomContext(&handler.Context{}))
	e.Use(middleware.Logging(log))
	e.Use(middleware.Panic(log))
	e.Use(middleware.Session(db, log, &middleware.SessionConfig{}))

	AddRoutes(e, db, config)

	execution.SignalStart(e, config.Server.ListenAddress, &execution.Config{
		ReloadFunc: func() {
			StartServer(configPath)
			logging.Stop()
		},
		StopFunc: func() {
			logging.Stop()
		},
		TerminateFunc: func() {
			logging.Stop()
		},
	})
}
