package main

import (
	"context"
	"fmt"
	"log"

	"github.com/spf13/viper"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type Person struct {
	Name string
	Age  int
}

func init() {
	viper.SetConfigFile(`.env.json`)
	err := viper.ReadInConfig()
	if err != nil {
		panic(err)
	}

	if viper.GetBool(`debug`) {
		log.Println("Service RUN on DEBUG mode")
	}
}

func main() {
	// 建立 MongoDB 連線
	clientOptions := options.Client().ApplyURI(viper.GetString(`database.connection_string`))
	client, err := mongo.Connect(context.Background(), clientOptions)
	if err != nil {
		log.Fatal(err)
	}

	// 關閉連線
	defer func() {
		err := client.Disconnect(context.Background())
		if err != nil {
			log.Fatal(err)
		}
	}()

	// 選擇資料庫與集合
	collection := client.Database(viper.GetString(`database.name`)).Collection("people")

	// 插入資料
	person := Person{"John Doe", 30}
	insertResult, err := collection.InsertOne(context.Background(), person)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Inserted document with ID:", insertResult.InsertedID)

	// 查詢資料
	filter := bson.M{"name": "John Doe"}
	var result Person
	err = collection.FindOne(context.Background(), filter).Decode(&result)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Found document:", result)
}
