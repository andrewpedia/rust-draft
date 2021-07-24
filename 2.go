package main

import (
	"io"
	"os"
	//	 "errors"
	"fmt"
)

func check(e error) {
	if e != nil {
		panic(e)
	}

}

func do(ww io.Writer) {
	doo(ww)
}

func doo(oo interface{}) {
	fmt.Printf("%T %T", 123, oo)

}

func main() {
	f, err := os.Create("dat2")
	check(err)

	defer f.Close()

	do(f)
}
