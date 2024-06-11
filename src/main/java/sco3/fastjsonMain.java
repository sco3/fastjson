package sco3;

import static java.lang.System.currentTimeMillis;
import static java.lang.System.getProperty;
import static java.lang.System.nanoTime;
import static java.lang.System.out;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;
import java.util.concurrent.atomic.AtomicInteger;

import com.alibaba.fastjson2.JSON;
import com.alibaba.fastjson2.JSONObject;

public class fastjsonMain {
	private static final String FILE = "~/.local/devices.json";
	private static final String DEVICES_JSON = ( //
	FILE.replaceFirst("^~", getProperty("user.home"))//
	);

	public static List<String> traverse() {
		return null;
	}

	public static void readWrite() throws IOException {
		String str = Files.readString(Paths.get(DEVICES_JSON));
		JSONObject o = JSON.parseObject(str);
		assert o != null;
	}

	public static void main(String[] argv) throws IOException {
		out.println("" //
				+ fastjsonMain.class.getSimpleName() //
				+ ": " + getProperty("java.vm.name") //
				+ " " + getProperty("java.vendor.version") //
		);

		long start = currentTimeMillis();
		Map<Long, AtomicInteger> timings = ( //
		new TreeMap<>() //
		);

		int n = 100;
		for (int i = 0; i < n; i++) {
			long singleStart = nanoTime();
			readWrite();
			long singleEnd = nanoTime();
			long dur = singleEnd - singleStart;
			// out.println("Took: " + dur + " ms.");
			AtomicInteger count = timings.get(dur);
			if (count == null) {
				count = new AtomicInteger(1);
				timings.put(dur, count);
			} else {
				count.incrementAndGet();
			}

		}
		long end = currentTimeMillis();
		long dur = end - start;
		out.println("Took: " + dur + " ms. (" + 1.0 * dur / n + " ms per run )");

		int sum = 0;
		for (long i : timings.keySet()) {
			sum += timings.get(i).get();
			if (sum >= n / 2) {
				out.println("Median time: " + 1.0 * i / 1000000 + " ms.");
				break;
			}
		}
	}
}