package main

import (
	"bufio"
	"fmt"
	"os"
	"runtime"
	"strings"
	"sync"
)

func worker(tasks <-chan string, wordsChan chan<- string, errorsChan chan<- error, wg *sync.WaitGroup) {
	defer wg.Done()

	for filename := range tasks {
		file, err := os.Open(filename)

		if err != nil {
			errorsChan <- err
			continue
		}

		scanner := bufio.NewScanner(file)
		scanner.Split(bufio.ScanWords)

		for scanner.Scan() {
			word := strings.ToLower(scanner.Text())

			wordsChan <- word
		}

		if scanner.Err() != nil {
			errorsChan <- scanner.Err()
		}

		file.Close()
	}
}

func collectResults(m map[string]int, wordsChan <-chan string, errorsChan <-chan error) {
	for {
		select {
		case word := <-wordsChan:
			if _, ok := m[word]; ok {
				m[word] += 1
			} else {
				m[word] = 1
			}
		case err := <-errorsChan:
			fmt.Fprintf(os.Stderr, "Error: %s\n", err)
		}
	}
}

func main() {
	numArgs := len(os.Args) - 1

	jobs := make(chan string, numArgs)
	wordsChan := make(chan string, numArgs)
	errorsChan := make(chan error, numArgs)

	wordCount := make(map[string]int)
	var wg sync.WaitGroup

	wg.Add(runtime.NumCPU())

	for w := 0; w < runtime.NumCPU(); w++ {
		go worker(jobs, wordsChan, errorsChan, &wg)
	}

	go collectResults(wordCount, wordsChan, errorsChan)

	for _, f := range os.Args[1:] {
		jobs <- f
	}

	close(jobs)

	wg.Wait()

	fmt.Println("Words that appear more than once:")
	for word, count := range wordCount {
		if count > 1 {
			fmt.Printf("%d %s\n", count, word)
		}
	}
}
