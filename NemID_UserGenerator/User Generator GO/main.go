package main

import (
	"encoding/json"
	"fmt"
	"github.com/gorilla/mux"
	"log"
	"math/rand"
	"net/http"
	"strconv"
	"strings"
)

type User struct {
	CPR   string `json:"cpr"`
	Email string `json:"email"`
}

type NemID struct {
	NemID string `json:nemId`
}

func main() {
	fmt.Println("NemID User Generator running on port 8088...")
	r := mux.NewRouter()
	r.HandleFunc("/generate-nemID", GenerateNemID).Methods("POST")
	log.Fatal(http.ListenAndServe(":8088", r))
}

func GenerateNemID(w http.ResponseWriter, r *http.Request) {
	var user User
	var response NemID
	var str strings.Builder

	w.Header().Set("Content-Type", "application/json")
	json.NewDecoder(r.Body).Decode(&user)

	for i := 0; i < 5; i++ {
		str.WriteString(strconv.Itoa(rand.Intn(9-0+1) + 0))
	}

	response.NemID = str.String() + "-" + string(user.CPR[len(user.CPR)-4:])
	json.NewEncoder(w).Encode(response)
}
