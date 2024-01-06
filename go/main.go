package main

import (
	"fmt"
	"net/http"
	"time"
)

func makeRequest(url string) (int64, error) {
	start := time.Now()

	resp, err := http.Get(url)
	if err != nil {
		return 0, err
	}
	defer resp.Body.Close()

	duration := time.Since(start).Milliseconds()
	return duration, nil
}

func main() {
	url := "https://example.com/"
	var totalDuration int64 = 0
	const numRequests = 100

	for i := 0; i < numRequests; i++ {
		duration, err := makeRequest(url)
		if err != nil {
			fmt.Printf("Request error: %s\n", err)
			return
		}
		totalDuration += duration
	}

	averageDuration := totalDuration / numRequests
	fmt.Printf("Ortalama İstek Süresi (%d iterasyon): %d ms\n", numRequests, averageDuration)
}
