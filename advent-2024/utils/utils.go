package utils

import (
	"log"
	"os"
)

func ReadFile(fileName string) string {
	content, err := os.ReadFile(fileName)
	if err != nil {
		log.Fatal(err)
	}

	return string(content)
}
