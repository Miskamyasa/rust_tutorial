package main

import (
	"fmt"
	"time"
)

var pl = fmt.Println

func main() {
	pl("Hello")
	var now = time.Now()
	loc, _ := time.LoadLocation("Asia/Tokyo")
	pl(now.In(loc).Format("2006-01-02 15:04:05"))
}
