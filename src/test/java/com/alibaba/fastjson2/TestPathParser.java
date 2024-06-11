package com.alibaba.fastjson2;

import org.junit.Test;

public class TestPathParser {

	@Test
	public void test() {
		JSONPathParser p = new JSONPathParser("http://localhost");
		p.parse();

	}

}
