package main

import (
	"net/http"
	"server/core"
	"server/web"
)

func main() {
	logger := core.NewLogger()
	logger.Info("initializing dependencies")

	// Core application functionalities initialization
	app := &core.Application{Log: logger}
	router := web.NewRouter(app)

	// Starting server functionalities
	app.Log.Info("starting server")
	err := http.ListenAndServe(":3500", router.HandleRequests())

	if err != nil {
		app.Log.Error("error during server start-up. reason: " + err.Error())
		return
	}
}
