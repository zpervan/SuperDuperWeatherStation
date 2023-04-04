package database

import (
	"context"
	"fmt"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"os"
	"server/core"
	"time"
)

// Local environment variables
const databaseName = "sdws"

// Filter criteria
var noFilterCriteria = bson.M{}

// WeatherData Contains all necessary data in order to retrieve or store weather data
type WeatherData struct {
	Datetime    primitive.DateTime `json:"datetime" bson:"datetime"`
	Temperature float64            `json:"temperature" bson:"temperature"`
	Humidity    float64            `json:"humidity" bson:"humidity"`
}

// Database Contains all needed functionality and dependencies in order to execute database CRUD operations
type Database struct {
	app         *core.Application
	WeatherData *mongo.Collection
}

func NewDatabase(app *core.Application, dbUrl string) (*Database, error) {
	database := &Database{}
	database.app = app

	err := database.Connect(dbUrl)
	if err != nil {
		return nil, err
	}

	return database, nil
}

func (db *Database) Connect(dbUrl string) error {
	clientOptions := options.Client().ApplyURI(dbUrl)
	clientOptions.SetServerSelectionTimeout(3 * time.Second)
	client, err := mongo.Connect(context.TODO(), clientOptions)

	if err != nil {
		return err
	}

	// Check if the database connection is alive
	if err := client.Ping(context.TODO(), nil); err != nil {
		return err
	}

	// In a local dev environment (non-Docker), fetching the given environment variable
	// could return in an empty string and would not properly initialize the database
	database := os.Getenv("MONGO_INITDB_DATABASE")
	if database == "" {
		database = databaseName
	}

	db.WeatherData = client.Database(database).Collection("weather_data")

	db.app.Log.Info("database connection established")

	return nil
}

func (db *Database) Create(weatherData *WeatherData) error {
	// Ideally, this would be provided in the request data from the ESP01, but this can be accomplished only with addition
	// hardware modules - let's keep it simple and use the servers date-time functionality.
	weatherData.Datetime = primitive.NewDateTimeFromTime(time.Now())

	_, err := db.WeatherData.InsertOne(context.TODO(), weatherData)
	if err != nil {
		return err
	}

	db.app.Log.Info(fmt.Sprintf("creating new entry - temperature %s, humidity %s", weatherData.Temperature, weatherData.Humidity))

	return nil
}

func (db *Database) Get() ([]WeatherData, error) {
	cursor, err := db.WeatherData.Find(context.TODO(), noFilterCriteria)

	if err != nil {
		return nil, fmt.Errorf("could not retrieve all weather data. reason: %s", err)
	}

	var queryResults []WeatherData

	for cursor.Next(context.TODO()) {
		singleResult := WeatherData{}

		err := cursor.Decode(&singleResult)
		if err != nil {
			return nil, fmt.Errorf("could not retrieve weather data. reason: %s", err)
		}

		queryResults = append(queryResults, singleResult)
	}

	if len(queryResults) == 0 {
		db.app.Log.Warn("no tasks available")
	}

	return queryResults, nil
}
