package main

import (
	"fmt"
	"encoding/json"
	"log"
	"net/http"
	"github.com/gorilla/mux"
)

type User struct {
	CPR    string  `json:"cpr"`
	NemID   string  `json:"nemId"`
}

type Password struct {
	NemIdPassword string `json:"nemIdPassword"`
}

func GenereateNemIDPassword(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	var user User
	var password Password
	_ = json.NewDecoder(r.Body).Decode(&user)
	first2ofNemID := string(user.NemID[0:2])
	last2ofCPR := string(user.CPR[len(user.CPR)-2:])
	password.NemIdPassword = first2ofNemID + last2ofCPR
	json.NewEncoder(w).Encode(password)
}

func main() {
	fmt.Println("NemID Password generator running on port 8089...")
	r := mux.NewRouter()
	r.HandleFunc("/generate-password-nemID", GenereateNemIDPassword).Methods("POST")
	log.Fatal(http.ListenAndServe(":8089", r))
}
