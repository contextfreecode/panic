package main

import "fmt"

func main() {
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

var texts = []string{"smile", "tears"}

func processText(id int) string {
	text, err := retrieveText(id)
	if err != nil {
		panic(err)
	}
	codes := []rune(text)
	rotateBack(codes)
	return string(codes)
}

func retrieveText(id int) (string, error) {
	if id >= 1 && id <= len(texts) {
		return texts[id-1], nil
	}
	// return "", fmt.Errorf("404 - Not found: %d", id)
	return "", NotFoundError{Message: fmt.Sprintf("404 - Not found: %d", id)}
}

func rotateBack[T any](vals []T) {
	valsLen := len(vals)
	// if valsLen == 0 {
	// 	return
	// }
	first := vals[0]
	// copy(vals[:valsLen-1], vals[1:])
	for i := 1; i < valsLen; i += 1 {
		vals[i-1] = vals[i]
	}
	vals[valsLen-1] = first
}

type NotFoundError struct {
	Message string
}

func (e NotFoundError) Error() string {
	return e.Message
}
