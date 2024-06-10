package sco3;

import static io.avaje.jsonb.Jsonb.builder;
import static java.lang.System.currentTimeMillis;
import static java.lang.System.getProperty;
import static java.lang.System.out;
import static java.nio.file.Files.readAllBytes;
import static java.nio.file.Files.write;
import static java.nio.file.Paths.get;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.nio.file.Paths;
import java.util.Map;

import io.avaje.jsonb.JsonType;
import io.avaje.jsonb.Jsonb;

public class JsonbMain {
	private static final String FILE = "~/.local/devices.json";
	// private static final String FILE = "~/.local/d1.json";
	private static final String DEVICES_JSON = ( //
	FILE.replaceFirst("^~", getProperty("user.home"))//
	);

	@SuppressWarnings("unchecked")
	public static void main(String[] argv) throws IOException {
		out.println("" //
				+ JsonbMain.class.getSimpleName() //
				+ ": " + getProperty("java.vm.name") //
				+ " " + getProperty("java.vendor.version") //
		);

		long start = currentTimeMillis();
		byte[] bytes = readAllBytes(Paths.get(DEVICES_JSON));

		Jsonb b = builder() //
				.serializeNulls(true)//
				.serializeEmpty(true)//
				.build();

		@SuppressWarnings("rawtypes")
		JsonType<Map> type = b.type(Map.class);
		@SuppressWarnings("rawtypes")
		Map o = type.fromJson(bytes);
		o.put("returnCode", 1);
		ByteArrayOutputStream bout = new ByteArrayOutputStream();
		type.toJson(o, bout);
		write(get(DEVICES_JSON + ".java.out"), bout.toByteArray());
		long end = currentTimeMillis();
		out.println("Took: " + (end - start) + " ms.");
	}
}