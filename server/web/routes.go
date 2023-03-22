package web

import (
	"github.com/gorilla/mux"
	"net/http"
	"server/core"
)

type Routes struct {
	App      *core.Application
	Handlers *Handler
}

func NewRouter(app *core.Application) *Routes {
	routes := &Routes{}

	routes.App = app
	routes.Handlers = NewHandler(app)

	return routes
}

func (r *Routes) HandleRequests() *mux.Router {
	router := mux.NewRouter()

	// GET
	router.Path("/ping").Methods(http.MethodGet).HandlerFunc(r.Handlers.Ping)
	
	return router
}
