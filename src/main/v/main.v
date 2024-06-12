import x.json2
import os
import time

fn run() ! {
	home := os.home_dir()
	filename := home + '/.local/devices.json'
	outfilename := home + '/.local/devices-out-v.json'
	buf := os.read_file(filename)!
	v := json2.raw_decode(buf)!
	mut vmap := v.as_map()
	vmap['returnCode'] = 1
	os.write_file(outfilename, vmap.str())!
}

fn main() {
	mut start := time.now()
	n := 100
	for i := 0; i < n; i++ {
		run()!
	}
	took := time.since(start) / time.millisecond
	avg := f32(took) / n
	println('Time: ${took} ms avg: ${avg} runs: ${n}')
}
