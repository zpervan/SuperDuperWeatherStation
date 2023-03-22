package web

import (
	"encoding/json"
	"net/http"
	"server/core"
)

type Handler struct {
	App *core.Application
}

func NewHandler(app *core.Application) *Handler {
	handlers := &Handler{}
	handlers.App = app

	return handlers
}

func (h *Handler) Ping(w http.ResponseWriter, req *http.Request) {
	h.App.Log.Info("ping request")

	err := json.NewEncoder(w).Encode("Test string")
	if err != nil {
		h.App.Log.Warn("error while pinging. reason: " + err.Error())
	}
}
