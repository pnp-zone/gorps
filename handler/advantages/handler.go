package advantages

import (
	"github.com/labstack/echo/v4"
	"github.com/myOmikron/echotools/logging"
	"github.com/myOmikron/echotools/middleware"
	"github.com/pnp-zone/gorps/handler"
	"github.com/pnp-zone/gorps/models"
	"gorm.io/gorm"
)

var log = logging.GetLogger("advantage-handler")

type Handler struct {
	DB *gorm.DB
}

func (h *Handler) Get() echo.HandlerFunc {
	return middleware.Wrap(func(c *handler.Context) error {

		advantages := make([]models.Advantage, 0)
		if err := h.DB.Model(&advantages).Error; err != nil {
			log.Warn(err.Error())
			return c.JSON(500, &handler.Error{Error: "Database error"})
		}

		return c.JSON(200, &Data{Items: advantages})
	})
}
