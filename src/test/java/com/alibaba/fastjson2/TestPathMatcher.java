package com.alibaba.fastjson2;

import static org.apache.commons.io.FilenameUtils.wildcardMatch;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

public class TestPathMatcher {

	@Test
	public void test1() {
		boolean result = wildcardMatch(//
				"thermostat oic.r.humidity /humidity/indoor", //
				"thermostat oic.r.humidity *door"//
		);
		assertTrue(result);
	}

	@Test
	public void test2() {
		boolean result = wildcardMatch(//
				"thermostat oic.r.humidity /humidity/indoor", //
				"thermostat oic.r.humidity *outdoor"//
		);
		assertFalse(result);
	}

}
