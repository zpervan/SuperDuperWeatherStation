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
const dateFormatLayout = "20060102"

// Filter criteria
var noFilterCriteria = bson.M{}

// WeatherData Contains all necessary data in order to retrieve or store weather data
type WeatherData struct {
	CreatedOn   primitive.DateTime `json:"created_on" bson:"created_on"`
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
	// hardware modules - let's keep it simple and use the server's date-time functionality.
	weatherData.CreatedOn = primitive.NewDateTimeFromTime(time.Now())

	_, err := db.WeatherData.InsertOne(context.TODO(), weatherData)
	if err != nil {
		return err
	}

	db.app.Log.Info(fmt.Sprintf("creating new entry - temperature %s, humidity %s", weatherData.Temperature, weatherData.Humidity))

	return nil
}

func (db *Database) GetAllWeatherData() (queryResult []WeatherData, err error) {
	defer func() {
		if err != nil {
			db.app.Log.Error(fmt.Sprintf("could not fetch all weather data. reason: %s", err))
		}
	}()

	// Retrieved ALL the data located inside the database
	cursor, err := db.WeatherData.Find(context.TODO(), noFilterCriteria)

	if err != nil {
		return nil, fmt.Errorf("could not retrieve all weather data. reason: %s", err)
	}

	for cursor.Next(context.TODO()) {
		singleResult := WeatherData{}

		err := cursor.Decode(&singleResult)
		if err != nil {
			return nil, fmt.Errorf("could not retrieve weather data. reason: %s", err)
		}

		queryResult = append(queryResult, singleResult)
	}

	if len(queryResult) == 0 {
		db.app.Log.Warn("no weather data available")
	}

	return queryResult, nil
}

func (db *Database) GetByDate(date *string) (queryResult []WeatherData, err error) {
	defer func() {
		if err != nil {
			db.app.Log.Error(fmt.Sprintf("could not fetch data by date %s. reason: %s", date, err))
		}
	}()

	parsedDate, err := time.Parse(dateFormatLayout, *date)
	if err != nil {
		return nil, err
	}

	// A filter criteria which will fetch all data on the parsed date
	// We search between the days which are the day before and after the parsed date and obtain all data
	// If the $eq (equal) operator was used, it would only return one or none entries which has the given time component (i.e., 00:00:00)
	dateFilter := bson.M{"created_on": bson.M{
		"$gt": primitive.NewDateTimeFromTime(parsedDate).Time().AddDate(0, 0, -1),
		"$lt": primitive.NewDateTimeFromTime(parsedDate).Time().AddDate(0, 0, 1),
	}}

	cursor, err := db.WeatherData.Find(context.TODO(), dateFilter)
	if err != nil {
		return nil, err
	}

	for cursor.Next(context.TODO()) {
		singleResult := WeatherData{}

		err := cursor.Decode(&singleResult)
		if err != nil {
			return nil, fmt.Errorf("could not retrieve weather data by date. reason: %s", err)
		}

		queryResult = append(queryResult, singleResult)
	}

	if len(queryResult) == 0 {
		db.app.Log.Warn("no weather data with that date available")
	}

	return queryResult, nil
}

func (db *Database) GetDates() (_ []string, err error) {
	defer func() {
		if err != nil {
			db.app.Log.Error(fmt.Sprintf("could not get dates. reason: %s", err))
		}
	}()

	cursor, err := db.WeatherData.Distinct(context.TODO(), "created_on", noFilterCriteria)
	if err != nil {
		return nil, fmt.Errorf("could not retrieve dates. reason: %s", err)
	}

	queryResult := core.Set{}

	// Iterate through the created_at data column, convert the datetime field to a date and add unique dates to the result
	for _, date := range cursor {
		year, month, day := date.(primitive.DateTime).Time().Date()
		queryResult.Add(fmt.Sprintf("%d%02d%02d", year, int(month), day))
	}

	if len(queryResult) == 0 {
		db.app.Log.Warn("no dates available")
	}

	// Returning a slice so that the JSON encoder returns a list with elements, otherwise it would return a list with objects.
	// This makes the JSON encoded response cleaner and simpler.
	return queryResult.ToSlice(), nil
}

func (db *Database) GetLatestDate() (_ string, err error) {
	defer func() {
		if err != nil {
			db.app.Log.Error(fmt.Sprintf("could not get latest date. reason: %s", err))
		}
	}()

	// Sort the result by descending order base on the _id field
	latestEntryFilter := bson.D{{"_id", -1}}
	options := options.FindOne().SetSort(latestEntryFilter)

	queryResult := db.WeatherData.FindOne(context.TODO(), bson.M{}, options)
	err = queryResult.Err()
	if err != nil {
		return "", err
	}

	// Populate the query results into a WeatherData structure
	var weatherDataResult WeatherData
	err = queryResult.Decode(&weatherDataResult)
	if err != nil {
		return "", err
	}

	// Convert and format the latest date string
	year, month, day := weatherDataResult.CreatedOn.Time().Date()
	return fmt.Sprintf("%d%02d%02d", year, int(month), day), nil
}
