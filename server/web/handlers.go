package web

import (
	"encoding/json"
	"github.com/go-chi/chi/v5"
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

func (h *Handler) Ping(w http.ResponseWriter, _ *http.Request) {
	h.App.Log.Info("ping request")

	err := json.NewEncoder(w).Encode("OK")
	if err != nil {
		h.App.Log.Warn("error while pinging. reason: " + err.Error())
	}
}

func (h *Handler) Add(w http.ResponseWriter, req *http.Request) {
	h.App.Log.Info("adding new weather data")

	var weatherData database.WeatherData

	err := json.NewDecoder(req.Body).Decode(&weatherData)
	if err != nil {
		h.App.Log.Error("error while adding new weather data. reason: " + err.Error())
		return
	}

	err = h.Database.Create(&weatherData)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Header().Set("X-Status-Reason", "error while adding new weather data")
	}
}

func (h *Handler) GetAllWeatherData(w http.ResponseWriter, _ *http.Request) {
	h.App.Log.Info("getting weather data")

	weatherData, err := h.Database.GetAllWeatherData()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Header().Set("X-Status-Reason", "error while getting weather data")
		return
	}

	err = json.NewEncoder(w).Encode(weatherData)
	if err != nil {
		h.App.Log.Error("error while encoding fetched data. reason: " + err.Error())
	}
}

func (h *Handler) GetByDate(w http.ResponseWriter, req *http.Request) {
	date := chi.URLParam(req, "date")

	h.App.Log.Info("getting weather data by date " + date)

	weatherData, err := h.Database.GetByDate(&date)

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Header().Set("X-Status-Reason", "error while getting weather data by date")
		return
	}

	err = json.NewEncoder(w).Encode(weatherData)
	if err != nil {
		h.App.Log.Error("error while encoding fetched data by date. reason: " + err.Error())
	}
}

func (h *Handler) GetDates(w http.ResponseWriter, _ *http.Request) {
	h.App.Log.Info("getting dates")

	dates, err := h.Database.GetDates()

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Header().Set("X-Status-Reason", "error while getting weather data by date")
		return
	}

	err = json.NewEncoder(w).Encode(dates)
	if err != nil {
		h.App.Log.Error("error while encoding fetched data by date. reason: " + err.Error())
	}
}
