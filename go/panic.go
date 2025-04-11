package main

import "fmt"

func main() {
	run()
	fmt.Println("Still alive.")
}

func run() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("panic:", r)
		}
	}()
	for i := 1; i <= 3; i++ {
		text := processText(i)
		fmt.Println(text)
	}
}

var texts = []string{"tar", "flow"}

func processText(id int) string {
	text, err := retrieveText(id)
	if err != nil {
		panic(err)
	}
	codes := []rune(text)
	reverseInPlace(codes)
	return string(codes)
}

func retrieveText(id int) (string, error) {
	if id >= 1 && id <= len(texts) {
		return texts[id-1], nil
	}
	// return "", fmt.Errorf("404 - Not found: %d", id)
	return "", NotFoundError{Message: fmt.Sprintf("404 - Not found: %d", id)}
}

func reverseInPlace[T any](values []T) {
	valuesLen := len(values)
	for index := 0; index < valuesLen/2; index += 1 {
		reverseIndex := valuesLen - index - 1
		values[index], values[reverseIndex] =
			values[reverseIndex], values[index]
	}
}

type NotFoundError struct {
	Message string
}

func (e NotFoundError) Error() string {
	return e.Message
}
