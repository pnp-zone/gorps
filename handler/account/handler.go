package account

import (
	"github.com/labstack/echo/v4"
	"github.com/myOmikron/echotools/auth"
	"github.com/myOmikron/echotools/database"
	"github.com/myOmikron/echotools/logging"
	"github.com/myOmikron/echotools/middleware"
	"github.com/myOmikron/echotools/utility"
	"github.com/myOmikron/echotools/utilitymodels"
	"github.com/pnp-zone/gorps/handler"
	"gorm.io/gorm"
)

var log = logging.GetLogger("account-handler")

type Handler struct {
	DB *gorm.DB
}

func (a *Handler) Register() echo.HandlerFunc {
	return middleware.Wrap(func(c *handler.Context) error {
		var f RegisterRequest
		if err := utility.ValidateJsonForm(c, &f); err != nil {
			log.Infof("Login failed: %s", err.Error())
			return c.JSON(400, &handler.Error{Error: err.Error()})
		}

		var userCount int64
		a.DB.Find(&utilitymodels.User{}, "username = ?", *f.Username).Count(&userCount)
		if userCount > 0 {
			return c.NoContent(409)
		}

		if _, err := database.CreateUser(a.DB, *f.Username, *f.Password, nil, true); err != nil {
			return err
		}

		return c.NoContent(200)
	})
}

func (a *Handler) Login() echo.HandlerFunc {
	return middleware.Wrap(func(c *handler.Context) error {
		var f LoginRequest
		if err := utility.ValidateJsonForm(c, &f); err != nil {
			log.Infof("Login failed: %s", err.Error())
			return c.JSON(400, &handler.Error{Error: err.Error()})
		}

		if user, err := auth.Authenticate(a.DB, *f.Username, *f.Password); err != nil {
			log.Infof("Failed to login: %s", err.Error())
			return c.NoContent(401)
		} else {
			if err := middleware.Login(a.DB, user, c); err != nil {
				log.Warnf("Error while logging in: %s", err.Error())
				return c.JSON(500, &handler.Error{Error: err.Error()})
			}
		}

		return c.NoContent(200)
	})
}

func (a *Handler) Logout() echo.HandlerFunc {
	return middleware.Wrap(func(c *handler.Context) error {
		if !c.IsAuthenticated() {
			return c.NoContent(200)
		}

		if err := middleware.Logout(a.DB, c); err != nil {
			return c.JSON(400, &handler.Error{Error: err.Error()})
		}

		return c.NoContent(200)
	})
}
