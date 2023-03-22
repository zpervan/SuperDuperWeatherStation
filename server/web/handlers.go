package web

import (
	"encoding/json"
	"net/http"
	"os"
	"server/core"
	"server/database"
)

// For local (non-Docker) development/testing
const localDatabaseUrl = "mongodb://admin:password@localhost:27018"

type Handler struct {
	App      *core.Application
	Database *database.Database
}

func NewHandler(app *core.Application) *Handler {
	var databaseUrl string

	if os.Getenv("DB_URL") != "" {
		databaseUrl = os.Getenv("DB_URL")
	} else {
		databaseUrl = localDatabaseUrl
	}

	databaseTemp, err := database.NewDatabase(app, databaseUrl)
	if err != nil {
		app.Log.Warn("Could not connect to database. Reason: " + err.Error())
		panic(err)
	}

	handlers := &Handler{}
	handlers.App = app
	handlers.Database = databaseTemp

	return handlers
}

func (h *Handler) Ping(w http.ResponseWriter, req *http.Request) {
	h.App.Log.Info("ping request")

	err := json.NewEncoder(w).Encode("Test string")
	if err != nil {
		h.App.Log.Warn("error while pinging. reason: " + err.Error())
	}
}

func (h *Handler) AddWeatherData(w http.ResponseWriter, req *http.Request) {
	h.App.Log.Info("adding new weather data")

	var weatherData database.WeatherData

	err := json.NewDecoder(req.Body).Decode(&weatherData)
	if err != nil {
		h.App.Log.Warn("error while adding new weather data. reason: " + err.Error())
	}

	context := req.Context()

	err = h.Database.Create(context, &weatherData)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Header().Set("X-Status-Reason", "error while adding new weather data")
	}
}
