package main

import (
	"fmt"
	"os"
	"time"

	"github.com/valyala/fastjson"
)

func run() {
	home, err := os.UserHomeDir()
	if err == nil {
		b, err := os.ReadFile(home + "/.local/devices.json")
		if err == nil {
			str := string(b)
			//fmt.Println(str)
			var p fastjson.Parser
			v, err := p.Parse(str)
			if err == nil {
				const field = "returnCode"
				//fmt.Printf("%s: %v\n", field, v.GetInt(field))
				v.Set(field, fastjson.MustParse("1"))
				mod := v.String()
				out := "/.local/devices-out-go.json"
				err = os.WriteFile(home+out, []byte(mod), 0644)
				if err != nil {
					os.Exit(1)
				}
			}
		}
	}
}

func main() {
	start := time.Now()
	n := int64(1000)
	for i := int64(0); i < n; i++ {
		run()
	}
	took := time.Since(start).Milliseconds()
	avg := float64(took) / float64(n)
	fmt.Printf("Time: %d ms avg: %v runs: %d\n", took, avg, n)
}
