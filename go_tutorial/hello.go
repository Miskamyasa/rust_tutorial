package main

import (
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	appJSON := map[string]interface{}{
		"expo": map[string]interface{}{
			"version": "1.0.0",
			"ios": map[string]interface{}{
				"buildNumber": "1",
			},
			"android": map[string]interface{}{
				"versionCode": 1,
			},
		},
	}
	appJSONFile, err := os.ReadFile("app.json")
	check(err)
	json.Unmarshal(appJSONFile, &appJSON)

	packageJSON := map[string]interface{}{
		"version": "1.0.0",
	}
	packageJSONFile, err := os.ReadFile("package.json")
	check(err)
	json.Unmarshal(packageJSONFile, &packageJSON)

	version := appJSON["expo"].(map[string]interface{})["version"].(string)

	oldVerVec := strings.Split(version, ".")
	oldPatch, err := strconv.Atoi(oldVerVec[2])
	check(err)

	newVer := fmt.Sprintf(
		"%s.%s.%s",
		oldVerVec[0],
		oldVerVec[1],
		strconv.Itoa(1+oldPatch),
	)

	appJSON["expo"].(map[string]interface{})["version"] = newVer
	packageJSON["version"] = newVer

	build := appJSON["expo"].(map[string]interface{})["android"].(map[string]interface{})["versionCode"].(float64) + 1

	appJSON["expo"].(map[string]interface{})["android"].(map[string]interface{})["versionCode"] = build
	appJSON["expo"].(map[string]interface{})["ios"].(map[string]interface{})["buildNumber"] = strconv.Itoa(int(build))

	appJSONBytes, err := json.MarshalIndent(appJSON, "", "  ")
	check(err)

	err = os.WriteFile("app.json", appJSONBytes, 0644)
	check(err)

	packageJSONBytes, err := json.MarshalIndent(packageJSON, "", "  ")
	check(err)

	err = os.WriteFile("package.json", packageJSONBytes, 0644)
	check(err)

	fmt.Println("Done!")
}
