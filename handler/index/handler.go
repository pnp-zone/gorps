package index

import (
	"github.com/labstack/echo/v4"
	"github.com/myOmikron/echotools/middleware"
	"github.com/pnp-zone/gorps/handler"
)

type Model struct {
	IsAuthenticated bool
}

func Index() echo.HandlerFunc {
	return middleware.Wrap(func(c *handler.Context) error {
		return c.Render(200, "index", Model{IsAuthenticated: c.IsAuthenticated()})
	})
}
