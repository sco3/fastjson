package main

import (
	"fmt"
	"os"
	"time"

	"github.com/valyala/fastjson"
)

func main() {
	start := time.Now()
	home, err := os.UserHomeDir()
	if err == nil {
		b, err := os.ReadFile(home + "/.local/devices.json")
		if err == nil {
			str := string(b)
			fmt.Println(str)
			var p fastjson.Parser
			v, err := p.Parse(str)
			if err == nil {
				fmt.Printf("returnCode: %v\n", v.GetInt("returnCode"))
			}
		}
	}
	fmt.Printf("Time: %d ms\n", time.Since(start)/time.Millisecond)
}
