package sco3;

import static java.lang.System.currentTimeMillis;
import static java.lang.System.getProperty;
import static java.lang.System.nanoTime;
import static java.lang.System.out;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;
import java.util.concurrent.atomic.AtomicInteger;

import com.alibaba.fastjson2.JSON;
import com.alibaba.fastjson2.JSONArray;
import com.alibaba.fastjson2.JSONObject;

public class fastjsonMain {
	private static final String FILE = "~/.local/devices.json";
	private static final String DEVICES_JSON = ( //
	FILE.replaceFirst("^~", getProperty("user.home"))//
	);
	private static JSONObject EMPTY_O = new JSONObject();
	private static JSONArray EMPTY_A = new JSONArray();

	public static JSONObject getObj(JSONObject root, String name) {
		JSONObject result = EMPTY_O;
		Object o = root.get(name);
		if (o != null && o instanceof JSONObject) {
			result = (JSONObject) o;
		}
		return result;
	}

	public static JSONArray getArr(JSONObject root, String name) {
		JSONArray result = EMPTY_A;
		Object o = root.get(name);
		if (o != null && o instanceof JSONArray) {
			result = (JSONArray) o;
		}
		return result;
	}

	public static String getStr(JSONObject root, String name) {
		String result = "";
		Object o = root.get(name);
		if (o != null && o instanceof String) {
			result = (String) o;
		}
		return result;
	}

	public static List<String> traverse(JSONObject root) {
		List<String> result = new ArrayList<>();
		StringBuilder sb = new StringBuilder();
		JSONObject data = getObj(root, "data");
		JSONArray devices = getArr(data, "devices");
		for (Object oDev : devices) {
			if (oDev != null && oDev instanceof JSONObject) {
				JSONObject dev = (JSONObject) oDev;
				String type = getStr(dev, "type");
				JSONObject meta = getObj(dev, "smartHomeDeviceMetadata");
				JSONArray resources = getArr(meta, "resources");
				for (Object oRes : resources) {
					if (oRes != null && oRes instanceof JSONObject) {
						JSONObject res = (JSONObject) oRes;
						String href = getStr(res, "href");
						JSONArray rts = getArr(res, "rt");
						for (Object oRt : rts) {
							if (oRt != null && oRt instanceof String) {
								String rt = (String) oRt;
								sb.append(type);
								sb.append(" ");
								sb.append(rt);
								sb.append(" ");
								sb.append(href);
								result.add(sb.toString());
								sb.setLength(0);
							}
						}
					}
				}
			}
		}
		return result;
	}

	public static JSONObject read() throws IOException {
		String str = Files.readString(Paths.get(DEVICES_JSON));
		JSONObject root = JSON.parseObject(str);

		assert root != null;
		return root;
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
		List<String> matches = new ArrayList<>();

		int n = 1000;
		for (int i = 0; i < n; i++) {
			long singleStart = nanoTime();
			JSONObject root = read();
			matches = traverse(root);
			long singleEnd = nanoTime();
			long dur = singleEnd - singleStart;
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
		for (String match : matches) {
			out.println(match);
		}
		out.println("" //
				+ "Took: " + dur + " ms for " + n + " runs. (avg " //
				+ 1.0 * dur / n + " ms )" //
		);

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