package app

import (
	"github.com/labstack/echo/v4"
	"github.com/myOmikron/echotools/database"
	"github.com/myOmikron/echotools/execution"
	"github.com/myOmikron/echotools/logging"
	"github.com/myOmikron/echotools/middleware"
	"github.com/pnp-zone/gorps/handler"
	"gorm.io/driver/sqlite"
)

func StartServer() {
	logging.Initialize(&logging.Config{})
	log := logging.GetLogger("app")
	e := echo.New()

	driver := sqlite.Open("gorps-server.sqlite3")
	db := database.Initialize(driver)

	middleware.CustomContext(&handler.Context{DB: db})
	middleware.Logging(log)
	middleware.Panic(log)
	middleware.Session(db, log, &middleware.SessionConfig{})

	execution.SignalStart(e, "127.0.0.1:8080", &execution.Config{
		ReloadFunc: func() {
			StartServer()
		},
		StopFunc: func() {

		},
		TerminateFunc: func() {

		},
	})
}
