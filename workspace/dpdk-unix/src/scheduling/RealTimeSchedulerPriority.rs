// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Real-time scheduler priority, from 1 to 99.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize)]
#[repr(u32)]
pub enum RealTimeSchedulerPriority
{
	#[serde(rename = "1")] _1 = 1,

	#[serde(rename = "2")] _2 = 2,

	#[serde(rename = "3")] _3 = 3,

	#[serde(rename = "4")] _4 = 4,

	#[serde(rename = "5")] _5 = 5,

	#[serde(rename = "6")] _6 = 6,

	#[serde(rename = "7")] _7 = 7,

	#[serde(rename = "8")] _8 = 8,

	#[serde(rename = "9")] _9 = 9,

	#[serde(rename = "10")] _10 = 10,

	#[serde(rename = "11")] _11 = 11,

	#[serde(rename = "12")] _12 = 12,

	#[serde(rename = "13")] _13 = 13,

	#[serde(rename = "14")] _14 = 14,

	#[serde(rename = "15")] _15 = 15,

	#[serde(rename = "16")] _16 = 16,

	#[serde(rename = "17")] _17 = 17,

	#[serde(rename = "18")] _18 = 18,

	#[serde(rename = "19")] _19 = 19,

	#[serde(rename = "20")] _20 = 20,

	#[serde(rename = "21")] _21 = 21,

	#[serde(rename = "22")] _22 = 22,

	#[serde(rename = "23")] _23 = 23,

	#[serde(rename = "24")] _24 = 24,

	#[serde(rename = "25")] _25 = 25,

	#[serde(rename = "26")] _26 = 26,

	#[serde(rename = "27")] _27 = 27,

	#[serde(rename = "28")] _28 = 28,

	#[serde(rename = "29")] _29 = 29,

	#[serde(rename = "30")] _30 = 30,

	#[serde(rename = "31")] _31 = 31,

	#[serde(rename = "32")] _32 = 32,

	#[serde(rename = "33")] _33 = 33,

	#[serde(rename = "34")] _34 = 34,

	#[serde(rename = "35")] _35 = 35,

	#[serde(rename = "36")] _36 = 36,

	#[serde(rename = "37")] _37 = 37,

	#[serde(rename = "38")] _38 = 38,

	#[serde(rename = "39")] _39 = 39,

	#[serde(rename = "40")] _40 = 40,

	#[serde(rename = "41")] _41 = 41,

	#[serde(rename = "42")] _42 = 42,

	#[serde(rename = "43")] _43 = 43,

	#[serde(rename = "44")] _44 = 44,

	#[serde(rename = "45")] _45 = 45,

	#[serde(rename = "46")] _46 = 46,

	#[serde(rename = "47")] _47 = 47,

	#[serde(rename = "48")] _48 = 48,

	#[serde(rename = "49")] _49 = 49,

	#[serde(rename = "50")] _50 = 50,

	#[serde(rename = "51")] _51 = 51,

	#[serde(rename = "52")] _52 = 52,

	#[serde(rename = "53")] _53 = 53,

	#[serde(rename = "54")] _54 = 54,

	#[serde(rename = "55")] _55 = 55,

	#[serde(rename = "56")] _56 = 56,

	#[serde(rename = "57")] _57 = 57,

	#[serde(rename = "58")] _58 = 58,

	#[serde(rename = "59")] _59 = 59,

	#[serde(rename = "60")] _60 = 60,

	#[serde(rename = "61")] _61 = 61,

	#[serde(rename = "62")] _62 = 62,

	#[serde(rename = "63")] _63 = 63,

	#[serde(rename = "64")] _64 = 64,

	#[serde(rename = "65")] _65 = 65,

	#[serde(rename = "66")] _66 = 66,

	#[serde(rename = "67")] _67 = 67,

	#[serde(rename = "68")] _68 = 68,

	#[serde(rename = "69")] _69 = 69,

	#[serde(rename = "70")] _70 = 70,

	#[serde(rename = "71")] _71 = 71,

	#[serde(rename = "72")] _72 = 72,

	#[serde(rename = "73")] _73 = 73,

	#[serde(rename = "74")] _74 = 74,

	#[serde(rename = "75")] _75 = 75,

	#[serde(rename = "76")] _76 = 76,

	#[serde(rename = "77")] _77 = 77,

	#[serde(rename = "78")] _78 = 78,

	#[serde(rename = "79")] _79 = 79,

	#[serde(rename = "80")] _80 = 80,

	#[serde(rename = "81")] _81 = 81,

	#[serde(rename = "82")] _82 = 82,

	#[serde(rename = "83")] _83 = 83,

	#[serde(rename = "84")] _84 = 84,

	#[serde(rename = "85")] _85 = 85,

	#[serde(rename = "86")] _86 = 86,

	#[serde(rename = "87")] _87 = 87,

	#[serde(rename = "88")] _88 = 88,

	#[serde(rename = "89")] _89 = 89,

	#[serde(rename = "90")] _90 = 90,

	#[serde(rename = "91")] _91 = 91,

	#[serde(rename = "92")] _92 = 92,

	#[serde(rename = "93")] _93 = 93,

	#[serde(rename = "94")] _94 = 94,

	#[serde(rename = "95")] _95 = 95,

	#[serde(rename = "96")] _96 = 96,

	#[serde(rename = "97")] _97 = 97,

	#[serde(rename = "98")] _98 = 98,

	#[serde(rename = "99")] _99 = 99,
}

impl Default for RealTimeSchedulerPriority
{
	#[inline(always)]
	fn default() -> Self
	{
		RealTimeSchedulerPriority::_99
	}
}
