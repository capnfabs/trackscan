mod data;

use data::stats::CatCount;

fn thing() -> Vec<(&'static str, CatCount)> {
    vec![(
        "1989-00",
        CatCount {
            counts: vec![
                128,
                0,
                0,
                2,
            ],
            total: 130,
        },
    ),
    (
        "1989-01",
        CatCount {
            counts: vec![
                1133,
                9,
                1,
                303,
            ],
            total: 1446,
        },
    ),
    (
        "1989-02",
        CatCount {
            counts: vec![
                1463,
                12,
                0,
                201,
            ],
            total: 1676,
        },
    ),
    (
        "1989-03",
        CatCount {
            counts: vec![
                1862,
                22,
                3,
                326,
            ],
            total: 2213,
        },
    ),
    (
        "1989-04",
        CatCount {
            counts: vec![
                2069,
                28,
                3,
                334,
            ],
            total: 2434,
        },
    ),
    (
        "1989-05",
        CatCount {
            counts: vec![
                1777,
                17,
                5,
                244,
            ],
            total: 2043,
        },
    ),
    (
        "1989-06",
        CatCount {
            counts: vec![
                1893,
                14,
                2,
                320,
            ],
            total: 2229,
        },
    ),
    (
        "1989-07",
        CatCount {
            counts: vec![
                1350,
                7,
                1,
                319,
            ],
            total: 1677,
        },
    ),
    (
        "1989-08",
        CatCount {
            counts: vec![
                1429,
                7,
                1,
                279,
            ],
            total: 1716,
        },
    ),
    (
        "1989-09",
        CatCount {
            counts: vec![
                1951,
                17,
                0,
                322,
            ],
            total: 2290,
        },
    ),
    (
        "1989-10",
        CatCount {
            counts: vec![
                3031,
                28,
                1,
                340,
            ],
            total: 3400,
        },
    ),
    (
        "1989-11",
        CatCount {
            counts: vec![
                2615,
                21,
                1,
                411,
            ],
            total: 3048,
        },
    ),
    (
        "1989-12",
        CatCount {
            counts: vec![
                1688,
                30,
                2,
                416,
            ],
            total: 2136,
        },
    ),
    (
        "1990-01",
        CatCount {
            counts: vec![
                1493,
                20,
                0,
                212,
            ],
            total: 1725,
        },
    ),
    (
        "1990-02",
        CatCount {
            counts: vec![
                1755,
                16,
                4,
                196,
            ],
            total: 1971,
        },
    ),
    (
        "1990-03",
        CatCount {
            counts: vec![
                1952,
                23,
                0,
                405,
            ],
            total: 2380,
        },
    ),
    (
        "1990-04",
        CatCount {
            counts: vec![
                1867,
                14,
                2,
                167,
            ],
            total: 2050,
        },
    ),
    (
        "1990-05",
        CatCount {
            counts: vec![
                1806,
                20,
                1,
                273,
            ],
            total: 2100,
        },
    ),
    (
        "1990-06",
        CatCount {
            counts: vec![
                2033,
                13,
                2,
                217,
            ],
            total: 2265,
        },
    ),
    (
        "1990-07",
        CatCount {
            counts: vec![
                1625,
                24,
                1,
                299,
            ],
            total: 1949,
        },
    ),
    (
        "1990-08",
        CatCount {
            counts: vec![
                1520,
                10,
                0,
                301,
            ],
            total: 1831,
        },
    ),
    (
        "1990-09",
        CatCount {
            counts: vec![
                2482,
                17,
                3,
                264,
            ],
            total: 2766,
        },
    ),
    (
        "1990-10",
        CatCount {
            counts: vec![
                3164,
                15,
                1,
                362,
            ],
            total: 3542,
        },
    ),
    (
        "1990-11",
        CatCount {
            counts: vec![
                2973,
                17,
                8,
                274,
            ],
            total: 3272,
        },
    ),
    (
        "1990-12",
        CatCount {
            counts: vec![
                1996,
                16,
                1,
                283,
            ],
            total: 2296,
        },
    ),
    (
        "1991-00",
        CatCount {
            counts: vec![
                260,
                9,
                0,
                0,
            ],
            total: 269,
        },
    ),
    (
        "1991-01",
        CatCount {
            counts: vec![
                1433,
                14,
                0,
                178,
            ],
            total: 1625,
        },
    ),
    (
        "1991-02",
        CatCount {
            counts: vec![
                1713,
                19,
                0,
                168,
            ],
            total: 1900,
        },
    ),
    (
        "1991-03",
        CatCount {
            counts: vec![
                1955,
                16,
                0,
                209,
            ],
            total: 2180,
        },
    ),
    (
        "1991-04",
        CatCount {
            counts: vec![
                2299,
                10,
                0,
                183,
            ],
            total: 2492,
        },
    ),
    (
        "1991-05",
        CatCount {
            counts: vec![
                2072,
                12,
                1,
                164,
            ],
            total: 2249,
        },
    ),
    (
        "1991-06",
        CatCount {
            counts: vec![
                2236,
                11,
                3,
                367,
            ],
            total: 2617,
        },
    ),
    (
        "1991-07",
        CatCount {
            counts: vec![
                2317,
                19,
                1,
                404,
            ],
            total: 2741,
        },
    ),
    (
        "1991-08",
        CatCount {
            counts: vec![
                1879,
                8,
                1,
                292,
            ],
            total: 2180,
        },
    ),
    (
        "1991-09",
        CatCount {
            counts: vec![
                2899,
                31,
                2,
                240,
            ],
            total: 3172,
        },
    ),
    (
        "1991-10",
        CatCount {
            counts: vec![
                3309,
                14,
                3,
                179,
            ],
            total: 3505,
        },
    ),
    (
        "1991-11",
        CatCount {
            counts: vec![
                3316,
                26,
                1,
                241,
            ],
            total: 3584,
        },
    ),
    (
        "1991-12",
        CatCount {
            counts: vec![
                2154,
                30,
                2,
                320,
            ],
            total: 2506,
        },
    ),
    (
        "1992-00",
        CatCount {
            counts: vec![
                501,
                3,
                0,
                2,
            ],
            total: 506,
        },
    ),
    (
        "1992-01",
        CatCount {
            counts: vec![
                1768,
                11,
                0,
                114,
            ],
            total: 1893,
        },
    ),
    (
        "1992-02",
        CatCount {
            counts: vec![
                2216,
                14,
                0,
                169,
            ],
            total: 2399,
        },
    ),
    (
        "1992-03",
        CatCount {
            counts: vec![
                2716,
                106,
                2,
                491,
            ],
            total: 3315,
        },
    ),
    (
        "1992-04",
        CatCount {
            counts: vec![
                2683,
                4,
                1,
                218,
            ],
            total: 2906,
        },
    ),
    (
        "1992-05",
        CatCount {
            counts: vec![
                2175,
                12,
                2,
                143,
            ],
            total: 2332,
        },
    ),
    (
        "1992-06",
        CatCount {
            counts: vec![
                2643,
                25,
                0,
                207,
            ],
            total: 2875,
        },
    ),
    (
        "1992-07",
        CatCount {
            counts: vec![
                2590,
                21,
                1,
                316,
            ],
            total: 2928,
        },
    ),
    (
        "1992-08",
        CatCount {
            counts: vec![
                2278,
                8,
                0,
                168,
            ],
            total: 2454,
        },
    ),
    (
        "1992-09",
        CatCount {
            counts: vec![
                2861,
                23,
                2,
                164,
            ],
            total: 3050,
        },
    ),
    (
        "1992-10",
        CatCount {
            counts: vec![
                3220,
                15,
                2,
                167,
            ],
            total: 3404,
        },
    ),
    (
        "1992-11",
        CatCount {
            counts: vec![
                3828,
                14,
                1,
                324,
            ],
            total: 4167,
        },
    ),
    (
        "1992-12",
        CatCount {
            counts: vec![
                2297,
                14,
                2,
                244,
            ],
            total: 2557,
        },
    ),
    (
        "1993-00",
        CatCount {
            counts: vec![
                877,
                12,
                0,
                5,
            ],
            total: 894,
        },
    ),
    (
        "1993-01",
        CatCount {
            counts: vec![
                1927,
                7,
                0,
                162,
            ],
            total: 2096,
        },
    ),
    (
        "1993-02",
        CatCount {
            counts: vec![
                2110,
                8,
                1,
                198,
            ],
            total: 2317,
        },
    ),
    (
        "1993-03",
        CatCount {
            counts: vec![
                3299,
                18,
                1,
                236,
            ],
            total: 3554,
        },
    ),
    (
        "1993-04",
        CatCount {
            counts: vec![
                2805,
                12,
                3,
                184,
            ],
            total: 3004,
        },
    ),
    (
        "1993-05",
        CatCount {
            counts: vec![
                2925,
                15,
                4,
                197,
            ],
            total: 3141,
        },
    ),
    (
        "1993-06",
        CatCount {
            counts: vec![
                3079,
                21,
                4,
                222,
            ],
            total: 3326,
        },
    ),
    (
        "1993-07",
        CatCount {
            counts: vec![
                2356,
                26,
                2,
                208,
            ],
            total: 2592,
        },
    ),
    (
        "1993-08",
        CatCount {
            counts: vec![
                2840,
                9,
                2,
                166,
            ],
            total: 3017,
        },
    ),
    (
        "1993-09",
        CatCount {
            counts: vec![
                3531,
                19,
                1,
                240,
            ],
            total: 3791,
        },
    ),
    (
        "1993-10",
        CatCount {
            counts: vec![
                4575,
                41,
                6,
                165,
            ],
            total: 4787,
        },
    ),
    (
        "1993-11",
        CatCount {
            counts: vec![
                4722,
                22,
                5,
                258,
            ],
            total: 5007,
        },
    ),
    (
        "1993-12",
        CatCount {
            counts: vec![
                2403,
                70,
                3,
                307,
            ],
            total: 2783,
        },
    ),
    (
        "1994-00",
        CatCount {
            counts: vec![
                946,
                11,
                1,
                17,
            ],
            total: 975,
        },
    ),
    (
        "1994-01",
        CatCount {
            counts: vec![
                1603,
                7,
                1,
                173,
            ],
            total: 1784,
        },
    ),
    (
        "1994-02",
        CatCount {
            counts: vec![
                2610,
                9,
                1,
                93,
            ],
            total: 2713,
        },
    ),
    (
        "1994-03",
        CatCount {
            counts: vec![
                3388,
                23,
                6,
                228,
            ],
            total: 3645,
        },
    ),
    (
        "1994-04",
        CatCount {
            counts: vec![
                3367,
                21,
                3,
                185,
            ],
            total: 3576,
        },
    ),
    (
        "1994-05",
        CatCount {
            counts: vec![
                3705,
                19,
                1,
                156,
            ],
            total: 3881,
        },
    ),
    (
        "1994-06",
        CatCount {
            counts: vec![
                3495,
                23,
                7,
                289,
            ],
            total: 3814,
        },
    ),
    (
        "1994-07",
        CatCount {
            counts: vec![
                2970,
                17,
                11,
                277,
            ],
            total: 3275,
        },
    ),
    (
        "1994-08",
        CatCount {
            counts: vec![
                3418,
                13,
                2,
                108,
            ],
            total: 3541,
        },
    ),
    (
        "1994-09",
        CatCount {
            counts: vec![
                3912,
                27,
                0,
                264,
            ],
            total: 4203,
        },
    ),
    (
        "1994-10",
        CatCount {
            counts: vec![
                5579,
                30,
                6,
                227,
            ],
            total: 5842,
        },
    ),
    (
        "1994-11",
        CatCount {
            counts: vec![
                3994,
                45,
                4,
                302,
            ],
            total: 4345,
        },
    ),
    (
        "1994-12",
        CatCount {
            counts: vec![
                2856,
                16,
                2,
                264,
            ],
            total: 3138,
        },
    ),
    (
        "1995-00",
        CatCount {
            counts: vec![
                1432,
                16,
                0,
                5,
            ],
            total: 1453,
        },
    ),
    (
        "1995-01",
        CatCount {
            counts: vec![
                3002,
                29,
                18,
                185,
            ],
            total: 3234,
        },
    ),
    (
        "1995-02",
        CatCount {
            counts: vec![
                3648,
                20,
                0,
                102,
            ],
            total: 3770,
        },
    ),
    (
        "1995-03",
        CatCount {
            counts: vec![
                3682,
                34,
                6,
                321,
            ],
            total: 4043,
        },
    ),
    (
        "1995-04",
        CatCount {
            counts: vec![
                4159,
                19,
                3,
                211,
            ],
            total: 4392,
        },
    ),
    (
        "1995-05",
        CatCount {
            counts: vec![
                4169,
                29,
                6,
                182,
            ],
            total: 4386,
        },
    ),
    (
        "1995-06",
        CatCount {
            counts: vec![
                4530,
                43,
                6,
                265,
            ],
            total: 4844,
        },
    ),
    (
        "1995-07",
        CatCount {
            counts: vec![
                3564,
                17,
                4,
                262,
            ],
            total: 3847,
        },
    ),
    (
        "1995-08",
        CatCount {
            counts: vec![
                4334,
                19,
                4,
                235,
            ],
            total: 4592,
        },
    ),
    (
        "1995-09",
        CatCount {
            counts: vec![
                4930,
                21,
                2,
                156,
            ],
            total: 5109,
        },
    ),
    (
        "1995-10",
        CatCount {
            counts: vec![
                6092,
                35,
                13,
                236,
            ],
            total: 6376,
        },
    ),
    (
        "1995-11",
        CatCount {
            counts: vec![
                5836,
                37,
                15,
                255,
            ],
            total: 6143,
        },
    ),
    (
        "1995-12",
        CatCount {
            counts: vec![
                3349,
                56,
                5,
                202,
            ],
            total: 3612,
        },
    ),
    (
        "1996-00",
        CatCount {
            counts: vec![
                1398,
                11,
                0,
                2,
            ],
            total: 1411,
        },
    ),
    (
        "1996-01",
        CatCount {
            counts: vec![
                3494,
                17,
                2,
                195,
            ],
            total: 3708,
        },
    ),
    (
        "1996-02",
        CatCount {
            counts: vec![
                4079,
                20,
                5,
                227,
            ],
            total: 4331,
        },
    ),
    (
        "1996-03",
        CatCount {
            counts: vec![
                4409,
                36,
                3,
                199,
            ],
            total: 4647,
        },
    ),
    (
        "1996-04",
        CatCount {
            counts: vec![
                5492,
                43,
                3,
                213,
            ],
            total: 5751,
        },
    ),
    (
        "1996-05",
        CatCount {
            counts: vec![
                4563,
                15,
                5,
                226,
            ],
            total: 4809,
        },
    ),
    (
        "1996-06",
        CatCount {
            counts: vec![
                5093,
                33,
                10,
                230,
            ],
            total: 5366,
        },
    ),
    (
        "1996-07",
        CatCount {
            counts: vec![
                5200,
                34,
                5,
                158,
            ],
            total: 5397,
        },
    ),
    (
        "1996-08",
        CatCount {
            counts: vec![
                4465,
                26,
                8,
                261,
            ],
            total: 4760,
        },
    ),
    (
        "1996-09",
        CatCount {
            counts: vec![
                6205,
                26,
                11,
                175,
            ],
            total: 6417,
        },
    ),
    (
        "1996-10",
        CatCount {
            counts: vec![
                6998,
                42,
                8,
                214,
            ],
            total: 7262,
        },
    ),
    (
        "1996-11",
        CatCount {
            counts: vec![
                6111,
                34,
                9,
                262,
            ],
            total: 6416,
        },
    ),
    (
        "1996-12",
        CatCount {
            counts: vec![
                3127,
                38,
                3,
                279,
            ],
            total: 3447,
        },
    ),
    (
        "1997-00",
        CatCount {
            counts: vec![
                1580,
                10,
                1,
                98,
            ],
            total: 1689,
        },
    ),
    (
        "1997-01",
        CatCount {
            counts: vec![
                3731,
                25,
                3,
                266,
            ],
            total: 4025,
        },
    ),
    (
        "1997-02",
        CatCount {
            counts: vec![
                3644,
                29,
                9,
                157,
            ],
            total: 3839,
        },
    ),
    (
        "1997-03",
        CatCount {
            counts: vec![
                4810,
                46,
                7,
                193,
            ],
            total: 5056,
        },
    ),
    (
        "1997-04",
        CatCount {
            counts: vec![
                4751,
                23,
                5,
                312,
            ],
            total: 5091,
        },
    ),
    (
        "1997-05",
        CatCount {
            counts: vec![
                4752,
                37,
                3,
                178,
            ],
            total: 4970,
        },
    ),
    (
        "1997-06",
        CatCount {
            counts: vec![
                5418,
                26,
                6,
                259,
            ],
            total: 5709,
        },
    ),
    (
        "1997-07",
        CatCount {
            counts: vec![
                4602,
                54,
                5,
                285,
            ],
            total: 4946,
        },
    ),
    (
        "1997-08",
        CatCount {
            counts: vec![
                4038,
                29,
                4,
                219,
            ],
            total: 4290,
        },
    ),
    (
        "1997-09",
        CatCount {
            counts: vec![
                6668,
                46,
                7,
                282,
            ],
            total: 7003,
        },
    ),
    (
        "1997-10",
        CatCount {
            counts: vec![
                6992,
                40,
                11,
                267,
            ],
            total: 7310,
        },
    ),
    (
        "1997-11",
        CatCount {
            counts: vec![
                6650,
                40,
                16,
                262,
            ],
            total: 6968,
        },
    ),
    (
        "1997-12",
        CatCount {
            counts: vec![
                3120,
                59,
                2,
                204,
            ],
            total: 3385,
        },
    ),
    (
        "1998-00",
        CatCount {
            counts: vec![
                1705,
                20,
                0,
                10,
            ],
            total: 1735,
        },
    ),
    (
        "1998-01",
        CatCount {
            counts: vec![
                3641,
                29,
                7,
                225,
            ],
            total: 3902,
        },
    ),
    (
        "1998-02",
        CatCount {
            counts: vec![
                4466,
                62,
                10,
                441,
            ],
            total: 4979,
        },
    ),
    (
        "1998-03",
        CatCount {
            counts: vec![
                5584,
                34,
                7,
                250,
            ],
            total: 5875,
        },
    ),
    (
        "1998-04",
        CatCount {
            counts: vec![
                5047,
                70,
                22,
                224,
            ],
            total: 5363,
        },
    ),
    (
        "1998-05",
        CatCount {
            counts: vec![
                4502,
                37,
                4,
                224,
            ],
            total: 4767,
        },
    ),
    (
        "1998-06",
        CatCount {
            counts: vec![
                6086,
                29,
                9,
                289,
            ],
            total: 6413,
        },
    ),
    (
        "1998-07",
        CatCount {
            counts: vec![
                4469,
                34,
                8,
                306,
            ],
            total: 4817,
        },
    ),
    (
        "1998-08",
        CatCount {
            counts: vec![
                4701,
                28,
                6,
                224,
            ],
            total: 4959,
        },
    ),
    (
        "1998-09",
        CatCount {
            counts: vec![
                6071,
                29,
                28,
                242,
            ],
            total: 6370,
        },
    ),
    (
        "1998-10",
        CatCount {
            counts: vec![
                6969,
                26,
                5,
                220,
            ],
            total: 7220,
        },
    ),
    (
        "1998-11",
        CatCount {
            counts: vec![
                7042,
                79,
                2,
                170,
            ],
            total: 7293,
        },
    ),
    (
        "1998-12",
        CatCount {
            counts: vec![
                3618,
                187,
                6,
                352,
            ],
            total: 4163,
        },
    ),
    (
        "1999-00",
        CatCount {
            counts: vec![
                2162,
                30,
                2,
                7,
            ],
            total: 2201,
        },
    ),
    (
        "1999-01",
        CatCount {
            counts: vec![
                3531,
                24,
                3,
                212,
            ],
            total: 3770,
        },
    ),
    (
        "1999-02",
        CatCount {
            counts: vec![
                4276,
                42,
                15,
                246,
            ],
            total: 4579,
        },
    ),
    (
        "1999-03",
        CatCount {
            counts: vec![
                6003,
                53,
                17,
                269,
            ],
            total: 6342,
        },
    ),
    (
        "1999-04",
        CatCount {
            counts: vec![
                5513,
                54,
                7,
                281,
            ],
            total: 5855,
        },
    ),
    (
        "1999-05",
        CatCount {
            counts: vec![
                6028,
                53,
                13,
                314,
            ],
            total: 6408,
        },
    ),
    (
        "1999-06",
        CatCount {
            counts: vec![
                5970,
                40,
                9,
                302,
            ],
            total: 6321,
        },
    ),
    (
        "1999-07",
        CatCount {
            counts: vec![
                4852,
                54,
                1,
                324,
            ],
            total: 5231,
        },
    ),
    (
        "1999-08",
        CatCount {
            counts: vec![
                5574,
                40,
                10,
                261,
            ],
            total: 5885,
        },
    ),
    (
        "1999-09",
        CatCount {
            counts: vec![
                7094,
                46,
                12,
                289,
            ],
            total: 7441,
        },
    ),
    (
        "1999-10",
        CatCount {
            counts: vec![
                8032,
                34,
                24,
                251,
            ],
            total: 8341,
        },
    ),
    (
        "1999-11",
        CatCount {
            counts: vec![
                7436,
                45,
                11,
                190,
            ],
            total: 7682,
        },
    ),
    (
        "1999-12",
        CatCount {
            counts: vec![
                4482,
                28,
                3,
                501,
            ],
            total: 5014,
        },
    ),
    (
        "2000-00",
        CatCount {
            counts: vec![
                2832,
                24,
                2,
                37,
            ],
            total: 2895,
        },
    ),
    (
        "2000-01",
        CatCount {
            counts: vec![
                4379,
                43,
                5,
                162,
            ],
            total: 4589,
        },
    ),
    (
        "2000-02",
        CatCount {
            counts: vec![
                4904,
                26,
                10,
                203,
            ],
            total: 5143,
        },
    ),
    (
        "2000-03",
        CatCount {
            counts: vec![
                5762,
                54,
                13,
                263,
            ],
            total: 6092,
        },
    ),
    (
        "2000-04",
        CatCount {
            counts: vec![
                5752,
                41,
                7,
                159,
            ],
            total: 5959,
        },
    ),
    (
        "2000-05",
        CatCount {
            counts: vec![
                6921,
                82,
                9,
                124,
            ],
            total: 7136,
        },
    ),
    (
        "2000-06",
        CatCount {
            counts: vec![
                6403,
                52,
                14,
                228,
            ],
            total: 6697,
        },
    ),
    (
        "2000-07",
        CatCount {
            counts: vec![
                5098,
                31,
                9,
                204,
            ],
            total: 5342,
        },
    ),
    (
        "2000-08",
        CatCount {
            counts: vec![
                5713,
                40,
                1,
                164,
            ],
            total: 5918,
        },
    ),
    (
        "2000-09",
        CatCount {
            counts: vec![
                7322,
                52,
                14,
                314,
            ],
            total: 7702,
        },
    ),
    (
        "2000-10",
        CatCount {
            counts: vec![
                8646,
                74,
                24,
                339,
            ],
            total: 9083,
        },
    ),
    (
        "2000-11",
        CatCount {
            counts: vec![
                7716,
                49,
                17,
                322,
            ],
            total: 8104,
        },
    ),
    (
        "2000-12",
        CatCount {
            counts: vec![
                4383,
                37,
                8,
                687,
            ],
            total: 5115,
        },
    ),
    (
        "2001-00",
        CatCount {
            counts: vec![
                2875,
                35,
                10,
                10,
            ],
            total: 2930,
        },
    ),
    (
        "2001-01",
        CatCount {
            counts: vec![
                4508,
                13,
                23,
                158,
            ],
            total: 4702,
        },
    ),
    (
        "2001-02",
        CatCount {
            counts: vec![
                6509,
                54,
                9,
                201,
            ],
            total: 6773,
        },
    ),
    (
        "2001-03",
        CatCount {
            counts: vec![
                7124,
                56,
                11,
                181,
            ],
            total: 7372,
        },
    ),
    (
        "2001-04",
        CatCount {
            counts: vec![
                6266,
                68,
                19,
                186,
            ],
            total: 6539,
        },
    ),
    (
        "2001-05",
        CatCount {
            counts: vec![
                7257,
                58,
                16,
                235,
            ],
            total: 7566,
        },
    ),
    (
        "2001-06",
        CatCount {
            counts: vec![
                7356,
                58,
                58,
                230,
            ],
            total: 7702,
        },
    ),
    (
        "2001-07",
        CatCount {
            counts: vec![
                6286,
                46,
                17,
                174,
            ],
            total: 6523,
        },
    ),
    (
        "2001-08",
        CatCount {
            counts: vec![
                5577,
                41,
                10,
                276,
            ],
            total: 5904,
        },
    ),
    (
        "2001-09",
        CatCount {
            counts: vec![
                7354,
                64,
                9,
                172,
            ],
            total: 7599,
        },
    ),
    (
        "2001-10",
        CatCount {
            counts: vec![
                9518,
                107,
                13,
                254,
            ],
            total: 9892,
        },
    ),
    (
        "2001-11",
        CatCount {
            counts: vec![
                8357,
                49,
                13,
                309,
            ],
            total: 8728,
        },
    ),
    (
        "2001-12",
        CatCount {
            counts: vec![
                4736,
                32,
                6,
                303,
            ],
            total: 5077,
        },
    ),
    (
        "2002-00",
        CatCount {
            counts: vec![
                1038,
                3,
                0,
                1,
            ],
            total: 1042,
        },
    ),
    (
        "2002-01",
        CatCount {
            counts: vec![
                4771,
                55,
                14,
                174,
            ],
            total: 5014,
        },
    ),
    (
        "2002-02",
        CatCount {
            counts: vec![
                5479,
                41,
                14,
                287,
            ],
            total: 5821,
        },
    ),
    (
        "2002-03",
        CatCount {
            counts: vec![
                7363,
                58,
                9,
                289,
            ],
            total: 7719,
        },
    ),
    (
        "2002-04",
        CatCount {
            counts: vec![
                6997,
                46,
                3,
                246,
            ],
            total: 7292,
        },
    ),
    (
        "2002-05",
        CatCount {
            counts: vec![
                7655,
                69,
                10,
                384,
            ],
            total: 8118,
        },
    ),
    (
        "2002-06",
        CatCount {
            counts: vec![
                7388,
                55,
                9,
                244,
            ],
            total: 7696,
        },
    ),
    (
        "2002-07",
        CatCount {
            counts: vec![
                7140,
                52,
                75,
                200,
            ],
            total: 7467,
        },
    ),
    (
        "2002-08",
        CatCount {
            counts: vec![
                6088,
                35,
                7,
                189,
            ],
            total: 6319,
        },
    ),
    (
        "2002-09",
        CatCount {
            counts: vec![
                8821,
                106,
                9,
                284,
            ],
            total: 9220,
        },
    ),
    (
        "2002-10",
        CatCount {
            counts: vec![
                9752,
                61,
                13,
                325,
            ],
            total: 10151,
        },
    ),
    (
        "2002-11",
        CatCount {
            counts: vec![
                9582,
                71,
                10,
                326,
            ],
            total: 9989,
        },
    ),
    (
        "2002-12",
        CatCount {
            counts: vec![
                4761,
                34,
                4,
                233,
            ],
            total: 5032,
        },
    ),
    (
        "2003-00",
        CatCount {
            counts: vec![
                628,
                6,
                0,
                1,
            ],
            total: 635,
        },
    ),
    (
        "2003-01",
        CatCount {
            counts: vec![
                4525,
                44,
                19,
                207,
            ],
            total: 4795,
        },
    ),
    (
        "2003-02",
        CatCount {
            counts: vec![
                6518,
                41,
                19,
                167,
            ],
            total: 6745,
        },
    ),
    (
        "2003-03",
        CatCount {
            counts: vec![
                8288,
                63,
                9,
                424,
            ],
            total: 8784,
        },
    ),
    (
        "2003-04",
        CatCount {
            counts: vec![
                8070,
                73,
                20,
                350,
            ],
            total: 8513,
        },
    ),
    (
        "2003-05",
        CatCount {
            counts: vec![
                8448,
                107,
                21,
                236,
            ],
            total: 8812,
        },
    ),
    (
        "2003-06",
        CatCount {
            counts: vec![
                8935,
                79,
                4,
                216,
            ],
            total: 9234,
        },
    ),
    (
        "2003-07",
        CatCount {
            counts: vec![
                8021,
                52,
                9,
                235,
            ],
            total: 8317,
        },
    ),
    (
        "2003-08",
        CatCount {
            counts: vec![
                8139,
                69,
                23,
                226,
            ],
            total: 8457,
        },
    ),
    (
        "2003-09",
        CatCount {
            counts: vec![
                10961,
                70,
                6,
                211,
            ],
            total: 11248,
        },
    ),
    (
        "2003-10",
        CatCount {
            counts: vec![
                11177,
                71,
                17,
                218,
            ],
            total: 11483,
        },
    ),
    (
        "2003-11",
        CatCount {
            counts: vec![
                10784,
                69,
                27,
                342,
            ],
            total: 11222,
        },
    ),
    (
        "2003-12",
        CatCount {
            counts: vec![
                6402,
                59,
                13,
                274,
            ],
            total: 6748,
        },
    ),
    (
        "2004-00",
        CatCount {
            counts: vec![
                477,
                2,
                0,
                1,
            ],
            total: 480,
        },
    ),
    (
        "2004-01",
        CatCount {
            counts: vec![
                5273,
                42,
                4,
                217,
            ],
            total: 5536,
        },
    ),
    (
        "2004-02",
        CatCount {
            counts: vec![
                7504,
                41,
                9,
                132,
            ],
            total: 7686,
        },
    ),
    (
        "2004-03",
        CatCount {
            counts: vec![
                9981,
                66,
                14,
                388,
            ],
            total: 10449,
        },
    ),
    (
        "2004-04",
        CatCount {
            counts: vec![
                9101,
                63,
                8,
                239,
            ],
            total: 9411,
        },
    ),
    (
        "2004-05",
        CatCount {
            counts: vec![
                9747,
                90,
                5,
                250,
            ],
            total: 10092,
        },
    ),
    (
        "2004-06",
        CatCount {
            counts: vec![
                9842,
                64,
                15,
                242,
            ],
            total: 10163,
        },
    ),
    (
        "2004-07",
        CatCount {
            counts: vec![
                8194,
                85,
                7,
                224,
            ],
            total: 8510,
        },
    ),
    (
        "2004-08",
        CatCount {
            counts: vec![
                7632,
                40,
                15,
                157,
            ],
            total: 7844,
        },
    ),
    (
        "2004-09",
        CatCount {
            counts: vec![
                11564,
                140,
                46,
                359,
            ],
            total: 12109,
        },
    ),
    (
        "2004-10",
        CatCount {
            counts: vec![
                12042,
                122,
                29,
                214,
            ],
            total: 12407,
        },
    ),
    (
        "2004-11",
        CatCount {
            counts: vec![
                12635,
                81,
                12,
                315,
            ],
            total: 13043,
        },
    ),
    (
        "2004-12",
        CatCount {
            counts: vec![
                6563,
                63,
                11,
                191,
            ],
            total: 6828,
        },
    ),
    (
        "2005-00",
        CatCount {
            counts: vec![
                337,
                3,
                0,
                2,
            ],
            total: 342,
        },
    ),
    (
        "2005-01",
        CatCount {
            counts: vec![
                7700,
                59,
                19,
                181,
            ],
            total: 7959,
        },
    ),
    (
        "2005-02",
        CatCount {
            counts: vec![
                8495,
                51,
                11,
                128,
            ],
            total: 8685,
        },
    ),
    (
        "2005-03",
        CatCount {
            counts: vec![
                9919,
                57,
                6,
                330,
            ],
            total: 10312,
        },
    ),
    (
        "2005-04",
        CatCount {
            counts: vec![
                10225,
                65,
                13,
                227,
            ],
            total: 10530,
        },
    ),
    (
        "2005-05",
        CatCount {
            counts: vec![
                11970,
                69,
                10,
                101,
            ],
            total: 12150,
        },
    ),
    (
        "2005-06",
        CatCount {
            counts: vec![
                11183,
                72,
                21,
                277,
            ],
            total: 11553,
        },
    ),
    (
        "2005-07",
        CatCount {
            counts: vec![
                8731,
                72,
                10,
                190,
            ],
            total: 9003,
        },
    ),
    (
        "2005-08",
        CatCount {
            counts: vec![
                9220,
                80,
                17,
                314,
            ],
            total: 9631,
        },
    ),
    (
        "2005-09",
        CatCount {
            counts: vec![
                12104,
                85,
                14,
                253,
            ],
            total: 12456,
        },
    ),
    (
        "2005-10",
        CatCount {
            counts: vec![
                15062,
                115,
                17,
                215,
            ],
            total: 15409,
        },
    ),
    (
        "2005-11",
        CatCount {
            counts: vec![
                12930,
                95,
                26,
                437,
            ],
            total: 13488,
        },
    ),
    (
        "2005-12",
        CatCount {
            counts: vec![
                8059,
                96,
                34,
                257,
            ],
            total: 8446,
        },
    ),
    (
        "2006-00",
        CatCount {
            counts: vec![
                484,
                1,
                0,
                1,
            ],
            total: 486,
        },
    ),
    (
        "2006-01",
        CatCount {
            counts: vec![
                8435,
                65,
                24,
                224,
            ],
            total: 8748,
        },
    ),
    (
        "2006-02",
        CatCount {
            counts: vec![
                9320,
                63,
                11,
                212,
            ],
            total: 9606,
        },
    ),
    (
        "2006-03",
        CatCount {
            counts: vec![
                12736,
                95,
                22,
                422,
            ],
            total: 13275,
        },
    ),
    (
        "2006-04",
        CatCount {
            counts: vec![
                11768,
                58,
                4,
                249,
            ],
            total: 12079,
        },
    ),
    (
        "2006-05",
        CatCount {
            counts: vec![
                13135,
                87,
                9,
                372,
            ],
            total: 13603,
        },
    ),
    (
        "2006-06",
        CatCount {
            counts: vec![
                12571,
                80,
                15,
                256,
            ],
            total: 12922,
        },
    ),
    (
        "2006-07",
        CatCount {
            counts: vec![
                9606,
                71,
                16,
                309,
            ],
            total: 10002,
        },
    ),
    (
        "2006-08",
        CatCount {
            counts: vec![
                9798,
                72,
                10,
                229,
            ],
            total: 10109,
        },
    ),
    (
        "2006-09",
        CatCount {
            counts: vec![
                13375,
                88,
                17,
                391,
            ],
            total: 13871,
        },
    ),
    (
        "2006-10",
        CatCount {
            counts: vec![
                16506,
                151,
                16,
                283,
            ],
            total: 16956,
        },
    ),
    (
        "2006-11",
        CatCount {
            counts: vec![
                14050,
                110,
                71,
                247,
            ],
            total: 14478,
        },
    ),
    (
        "2006-12",
        CatCount {
            counts: vec![
                9173,
                107,
                33,
                530,
            ],
            total: 9843,
        },
    ),
    (
        "2007-00",
        CatCount {
            counts: vec![
                352,
                2,
                0,
                1,
            ],
            total: 355,
        },
    ),
    (
        "2007-01",
        CatCount {
            counts: vec![
                8418,
                124,
                24,
                178,
            ],
            total: 8744,
        },
    ),
    (
        "2007-02",
        CatCount {
            counts: vec![
                10175,
                90,
                28,
                339,
            ],
            total: 10632,
        },
    ),
    (
        "2007-03",
        CatCount {
            counts: vec![
                11767,
                86,
                14,
                354,
            ],
            total: 12221,
        },
    ),
    (
        "2007-04",
        CatCount {
            counts: vec![
                12178,
                94,
                24,
                285,
            ],
            total: 12581,
        },
    ),
    (
        "2007-05",
        CatCount {
            counts: vec![
                12582,
                80,
                20,
                197,
            ],
            total: 12879,
        },
    ),
    (
        "2007-06",
        CatCount {
            counts: vec![
                13002,
                99,
                14,
                205,
            ],
            total: 13320,
        },
    ),
    (
        "2007-07",
        CatCount {
            counts: vec![
                11544,
                91,
                21,
                215,
            ],
            total: 11871,
        },
    ),
    (
        "2007-08",
        CatCount {
            counts: vec![
                9735,
                91,
                8,
                295,
            ],
            total: 10129,
        },
    ),
    (
        "2007-09",
        CatCount {
            counts: vec![
                13309,
                111,
                14,
                277,
            ],
            total: 13711,
        },
    ),
    (
        "2007-10",
        CatCount {
            counts: vec![
                17085,
                86,
                18,
                210,
            ],
            total: 17399,
        },
    ),
    (
        "2007-11",
        CatCount {
            counts: vec![
                15010,
                121,
                25,
                319,
            ],
            total: 15475,
        },
    ),
    (
        "2007-12",
        CatCount {
            counts: vec![
                9354,
                137,
                20,
                248,
            ],
            total: 9759,
        },
    ),
    (
        "2008-00",
        CatCount {
            counts: vec![
                286,
                0,
                0,
                1,
            ],
            total: 287,
        },
    ),
    (
        "2008-01",
        CatCount {
            counts: vec![
                9592,
                89,
                16,
                220,
            ],
            total: 9917,
        },
    ),
    (
        "2008-02",
        CatCount {
            counts: vec![
                11177,
                99,
                39,
                232,
            ],
            total: 11547,
        },
    ),
    (
        "2008-03",
        CatCount {
            counts: vec![
                13585,
                125,
                13,
                427,
            ],
            total: 14150,
        },
    ),
    (
        "2008-04",
        CatCount {
            counts: vec![
                13659,
                128,
                21,
                230,
            ],
            total: 14038,
        },
    ),
    (
        "2008-05",
        CatCount {
            counts: vec![
                14013,
                92,
                8,
                253,
            ],
            total: 14366,
        },
    ),
    (
        "2008-06",
        CatCount {
            counts: vec![
                13545,
                171,
                12,
                279,
            ],
            total: 14007,
        },
    ),
    (
        "2008-07",
        CatCount {
            counts: vec![
                11261,
                101,
                16,
                274,
            ],
            total: 11652,
        },
    ),
    (
        "2008-08",
        CatCount {
            counts: vec![
                11434,
                124,
                22,
                292,
            ],
            total: 11872,
        },
    ),
    (
        "2008-09",
        CatCount {
            counts: vec![
                15085,
                97,
                17,
                308,
            ],
            total: 15507,
        },
    ),
    (
        "2008-10",
        CatCount {
            counts: vec![
                17574,
                199,
                24,
                323,
            ],
            total: 18120,
        },
    ),
    (
        "2008-11",
        CatCount {
            counts: vec![
                15871,
                114,
                16,
                367,
            ],
            total: 16368,
        },
    ),
    (
        "2008-12",
        CatCount {
            counts: vec![
                11817,
                92,
                39,
                327,
            ],
            total: 12275,
        },
    ),
    (
        "2009-00",
        CatCount {
            counts: vec![
                137,
                2,
                1,
                1,
            ],
            total: 141,
        },
    ),
    (
        "2009-01",
        CatCount {
            counts: vec![
                9026,
                62,
                19,
                183,
            ],
            total: 9290,
        },
    ),
    (
        "2009-02",
        CatCount {
            counts: vec![
                11576,
                97,
                17,
                228,
            ],
            total: 11918,
        },
    ),
    (
        "2009-03",
        CatCount {
            counts: vec![
                16107,
                140,
                78,
                401,
            ],
            total: 16726,
        },
    ),
    (
        "2009-04",
        CatCount {
            counts: vec![
                13761,
                111,
                44,
                304,
            ],
            total: 14220,
        },
    ),
    (
        "2009-05",
        CatCount {
            counts: vec![
                15252,
                127,
                16,
                225,
            ],
            total: 15620,
        },
    ),
    (
        "2009-06",
        CatCount {
            counts: vec![
                15134,
                111,
                12,
                309,
            ],
            total: 15566,
        },
    ),
    (
        "2009-07",
        CatCount {
            counts: vec![
                11049,
                122,
                21,
                244,
            ],
            total: 11436,
        },
    ),
    (
        "2009-08",
        CatCount {
            counts: vec![
                11092,
                119,
                21,
                328,
            ],
            total: 11560,
        },
    ),
    (
        "2009-09",
        CatCount {
            counts: vec![
                16559,
                152,
                34,
                362,
            ],
            total: 17107,
        },
    ),
    (
        "2009-10",
        CatCount {
            counts: vec![
                18541,
                186,
                17,
                583,
            ],
            total: 19327,
        },
    ),
    (
        "2009-11",
        CatCount {
            counts: vec![
                17977,
                127,
                13,
                322,
            ],
            total: 18439,
        },
    ),
    (
        "2009-12",
        CatCount {
            counts: vec![
                12039,
                100,
                45,
                339,
            ],
            total: 12523,
        },
    ),
    (
        "2010-00",
        CatCount {
            counts: vec![
                106,
                2,
                1,
                0,
            ],
            total: 109,
        },
    ),
    (
        "2010-01",
        CatCount {
            counts: vec![
                11696,
                111,
                11,
                243,
            ],
            total: 12061,
        },
    ),
    (
        "2010-02",
        CatCount {
            counts: vec![
                12079,
                93,
                30,
                271,
            ],
            total: 12473,
        },
    ),
    (
        "2010-03",
        CatCount {
            counts: vec![
                16679,
                145,
                34,
                468,
            ],
            total: 17326,
        },
    ),
    (
        "2010-04",
        CatCount {
            counts: vec![
                15293,
                138,
                39,
                241,
            ],
            total: 15711,
        },
    ),
    (
        "2010-05",
        CatCount {
            counts: vec![
                15615,
                147,
                25,
                280,
            ],
            total: 16067,
        },
    ),
    (
        "2010-06",
        CatCount {
            counts: vec![
                15622,
                126,
                37,
                296,
            ],
            total: 16081,
        },
    ),
    (
        "2010-07",
        CatCount {
            counts: vec![
                13342,
                107,
                25,
                298,
            ],
            total: 13772,
        },
    ),
    (
        "2010-08",
        CatCount {
            counts: vec![
                12940,
                130,
                17,
                321,
            ],
            total: 13408,
        },
    ),
    (
        "2010-09",
        CatCount {
            counts: vec![
                16099,
                115,
                50,
                376,
            ],
            total: 16640,
        },
    ),
    (
        "2010-10",
        CatCount {
            counts: vec![
                22063,
                192,
                15,
                524,
            ],
            total: 22794,
        },
    ),
    (
        "2010-11",
        CatCount {
            counts: vec![
                19840,
                148,
                42,
                445,
            ],
            total: 20475,
        },
    ),
    (
        "2010-12",
        CatCount {
            counts: vec![
                12677,
                115,
                29,
                342,
            ],
            total: 13163,
        },
    ),
    (
        "2011-00",
        CatCount {
            counts: vec![
                210,
                2,
                0,
                1,
            ],
            total: 213,
        },
    ),
    (
        "2011-01",
        CatCount {
            counts: vec![
                12318,
                279,
                22,
                198,
            ],
            total: 12817,
        },
    ),
    (
        "2011-02",
        CatCount {
            counts: vec![
                15200,
                170,
                29,
                407,
            ],
            total: 15806,
        },
    ),
    (
        "2011-03",
        CatCount {
            counts: vec![
                16797,
                152,
                38,
                353,
            ],
            total: 17340,
        },
    ),
    (
        "2011-04",
        CatCount {
            counts: vec![
                18110,
                149,
                29,
                362,
            ],
            total: 18650,
        },
    ),
    (
        "2011-05",
        CatCount {
            counts: vec![
                19215,
                146,
                28,
                297,
            ],
            total: 19686,
        },
    ),
    (
        "2011-06",
        CatCount {
            counts: vec![
                17059,
                115,
                30,
                344,
            ],
            total: 17548,
        },
    ),
    (
        "2011-07",
        CatCount {
            counts: vec![
                13030,
                138,
                27,
                344,
            ],
            total: 13539,
        },
    ),
    (
        "2011-08",
        CatCount {
            counts: vec![
                14722,
                148,
                63,
                327,
            ],
            total: 15260,
        },
    ),
    (
        "2011-09",
        CatCount {
            counts: vec![
                19713,
                186,
                15,
                353,
            ],
            total: 20267,
        },
    ),
    (
        "2011-10",
        CatCount {
            counts: vec![
                20350,
                189,
                41,
                406,
            ],
            total: 20986,
        },
    ),
    (
        "2011-11",
        CatCount {
            counts: vec![
                23776,
                201,
                32,
                412,
            ],
            total: 24421,
        },
    ),
    (
        "2011-12",
        CatCount {
            counts: vec![
                16689,
                150,
                32,
                418,
            ],
            total: 17289,
        },
    ),
    (
        "2012-00",
        CatCount {
            counts: vec![
                167,
                2,
                0,
                0,
            ],
            total: 169,
        },
    ),
    (
        "2012-01",
        CatCount {
            counts: vec![
                13256,
                120,
                24,
                353,
            ],
            total: 13753,
        },
    ),
    (
        "2012-02",
        CatCount {
            counts: vec![
                15968,
                146,
                42,
                673,
            ],
            total: 16829,
        },
    ),
    (
        "2012-03",
        CatCount {
            counts: vec![
                19470,
                207,
                50,
                434,
            ],
            total: 20161,
        },
    ),
    (
        "2012-04",
        CatCount {
            counts: vec![
                20015,
                189,
                32,
                373,
            ],
            total: 20609,
        },
    ),
    (
        "2012-05",
        CatCount {
            counts: vec![
                20561,
                190,
                41,
                391,
            ],
            total: 21183,
        },
    ),
    (
        "2012-06",
        CatCount {
            counts: vec![
                18490,
                207,
                27,
                340,
            ],
            total: 19064,
        },
    ),
    (
        "2012-07",
        CatCount {
            counts: vec![
                17519,
                199,
                22,
                492,
            ],
            total: 18232,
        },
    ),
    (
        "2012-08",
        CatCount {
            counts: vec![
                15790,
                153,
                25,
                488,
            ],
            total: 16456,
        },
    ),
    (
        "2012-09",
        CatCount {
            counts: vec![
                19847,
                219,
                44,
                341,
            ],
            total: 20451,
        },
    ),
    (
        "2012-10",
        CatCount {
            counts: vec![
                25383,
                377,
                56,
                474,
            ],
            total: 26290,
        },
    ),
    (
        "2012-11",
        CatCount {
            counts: vec![
                22957,
                236,
                115,
                470,
            ],
            total: 23778,
        },
    ),
    (
        "2012-12",
        CatCount {
            counts: vec![
                18839,
                206,
                44,
                452,
            ],
            total: 19541,
        },
    ),
    (
        "2013-00",
        CatCount {
            counts: vec![
                67,
                4,
                2,
                1,
            ],
            total: 74,
        },
    ),
    (
        "2013-01",
        CatCount {
            counts: vec![
                14823,
                216,
                53,
                355,
            ],
            total: 15447,
        },
    ),
    (
        "2013-02",
        CatCount {
            counts: vec![
                17784,
                199,
                66,
                419,
            ],
            total: 18468,
        },
    ),
    (
        "2013-03",
        CatCount {
            counts: vec![
                20985,
                229,
                55,
                631,
            ],
            total: 21900,
        },
    ),
    (
        "2013-04",
        CatCount {
            counts: vec![
                22753,
                259,
                18,
                401,
            ],
            total: 23431,
        },
    ),
    (
        "2013-05",
        CatCount {
            counts: vec![
                21519,
                335,
                44,
                435,
            ],
            total: 22333,
        },
    ),
    (
        "2013-06",
        CatCount {
            counts: vec![
                20783,
                281,
                48,
                310,
            ],
            total: 21422,
        },
    ),
    (
        "2013-07",
        CatCount {
            counts: vec![
                17227,
                235,
                38,
                455,
            ],
            total: 17955,
        },
    ),
    (
        "2013-08",
        CatCount {
            counts: vec![
                16172,
                194,
                54,
                360,
            ],
            total: 16780,
        },
    ),
    (
        "2013-09",
        CatCount {
            counts: vec![
                23097,
                211,
                47,
                469,
            ],
            total: 23824,
        },
    ),
    (
        "2013-10",
        CatCount {
            counts: vec![
                27079,
                317,
                103,
                495,
            ],
            total: 27994,
        },
    ),
    (
        "2013-11",
        CatCount {
            counts: vec![
                27393,
                277,
                59,
                586,
            ],
            total: 28315,
        },
    ),
    (
        "2013-12",
        CatCount {
            counts: vec![
                18758,
                216,
                52,
                496,
            ],
            total: 19522,
        },
    ),
    (
        "2013-14",
        CatCount {
            counts: vec![
                3,
                0,
                0,
                0,
            ],
            total: 3,
        },
    ),
    (
        "2014-00",
        CatCount {
            counts: vec![
                50,
                0,
                0,
                1,
            ],
            total: 51,
        },
    ),
    (
        "2014-01",
        CatCount {
            counts: vec![
                16236,
                236,
                50,
                395,
            ],
            total: 16917,
        },
    ),
    (
        "2014-02",
        CatCount {
            counts: vec![
                20391,
                239,
                35,
                472,
            ],
            total: 21137,
        },
    ),
    (
        "2014-03",
        CatCount {
            counts: vec![
                22087,
                282,
                126,
                575,
            ],
            total: 23070,
        },
    ),
    (
        "2014-04",
        CatCount {
            counts: vec![
                24729,
                275,
                55,
                631,
            ],
            total: 25690,
        },
    ),
    (
        "2014-05",
        CatCount {
            counts: vec![
                21735,
                242,
                24,
                507,
            ],
            total: 22508,
        },
    ),
    (
        "2014-06",
        CatCount {
            counts: vec![
                21611,
                271,
                70,
                519,
            ],
            total: 22471,
        },
    ),
    (
        "2014-07",
        CatCount {
            counts: vec![
                18258,
                242,
                81,
                497,
            ],
            total: 19078,
        },
    ),
    (
        "2014-08",
        CatCount {
            counts: vec![
                17736,
                256,
                49,
                580,
            ],
            total: 18621,
        },
    ),
    (
        "2014-09",
        CatCount {
            counts: vec![
                25203,
                321,
                53,
                595,
            ],
            total: 26172,
        },
    ),
    (
        "2014-10",
        CatCount {
            counts: vec![
                27865,
                366,
                58,
                611,
            ],
            total: 28900,
        },
    ),
    (
        "2014-11",
        CatCount {
            counts: vec![
                27244,
                321,
                39,
                578,
            ],
            total: 28182,
        },
    ),
    (
        "2014-12",
        CatCount {
            counts: vec![
                20816,
                329,
                60,
                849,
            ],
            total: 22054,
        },
    ),
    (
        "2015-00",
        CatCount {
            counts: vec![
                64,
                0,
                0,
                1,
            ],
            total: 65,
        },
    ),
    (
        "2015-01",
        CatCount {
            counts: vec![
                17516,
                285,
                62,
                575,
            ],
            total: 18438,
        },
    ),
    (
        "2015-02",
        CatCount {
            counts: vec![
                20474,
                281,
                193,
                504,
            ],
            total: 21452,
        },
    ),
    (
        "2015-03",
        CatCount {
            counts: vec![
                24963,
                292,
                140,
                613,
            ],
            total: 26008,
        },
    ),
    (
        "2015-04",
        CatCount {
            counts: vec![
                23876,
                292,
                69,
                588,
            ],
            total: 24825,
        },
    ),
    (
        "2015-05",
        CatCount {
            counts: vec![
                23133,
                309,
                80,
                436,
            ],
            total: 23958,
        },
    ),
    (
        "2015-06",
        CatCount {
            counts: vec![
                22939,
                369,
                75,
                409,
            ],
            total: 23792,
        },
    ),
    (
        "2015-07",
        CatCount {
            counts: vec![
                18475,
                238,
                69,
                538,
            ],
            total: 19320,
        },
    ),
    (
        "2015-08",
        CatCount {
            counts: vec![
                17657,
                243,
                72,
                437,
            ],
            total: 18409,
        },
    ),
    (
        "2015-09",
        CatCount {
            counts: vec![
                24514,
                338,
                63,
                446,
            ],
            total: 25361,
        },
    ),
    (
        "2015-10",
        CatCount {
            counts: vec![
                31371,
                472,
                225,
                428,
            ],
            total: 32496,
        },
    ),
    (
        "2015-11",
        CatCount {
            counts: vec![
                27191,
                315,
                75,
                560,
            ],
            total: 28141,
        },
    ),
    (
        "2015-12",
        CatCount {
            counts: vec![
                22094,
                262,
                105,
                568,
            ],
            total: 23029,
        },
    ),
    (
        "2015-13",
        CatCount {
            counts: vec![
                9,
                0,
                0,
                0,
            ],
            total: 9,
        },
    ),
    (
        "2016-00",
        CatCount {
            counts: vec![
                32,
                1,
                0,
                0,
            ],
            total: 33,
        },
    ),
    (
        "2016-01",
        CatCount {
            counts: vec![
                18588,
                242,
                104,
                531,
            ],
            total: 19465,
        },
    ),
    (
        "2016-02",
        CatCount {
            counts: vec![
                22229,
                375,
                116,
                552,
            ],
            total: 23272,
        },
    ),
    (
        "2016-03",
        CatCount {
            counts: vec![
                23771,
                369,
                95,
                597,
            ],
            total: 24832,
        },
    ),
    (
        "2016-04",
        CatCount {
            counts: vec![
                27695,
                377,
                94,
                552,
            ],
            total: 28718,
        },
    ),
    (
        "2016-05",
        CatCount {
            counts: vec![
                24304,
                313,
                141,
                445,
            ],
            total: 25203,
        },
    ),
    (
        "2016-06",
        CatCount {
            counts: vec![
                22951,
                404,
                94,
                475,
            ],
            total: 23924,
        },
    ),
    (
        "2016-07",
        CatCount {
            counts: vec![
                20414,
                376,
                128,
                541,
            ],
            total: 21459,
        },
    ),
    (
        "2016-08",
        CatCount {
            counts: vec![
                18703,
                226,
                69,
                372,
            ],
            total: 19370,
        },
    ),
    (
        "2016-09",
        CatCount {
            counts: vec![
                28581,
                378,
                131,
                580,
            ],
            total: 29670,
        },
    ),
    (
        "2016-10",
        CatCount {
            counts: vec![
                31319,
                342,
                121,
                522,
            ],
            total: 32304,
        },
    ),
    (
        "2016-11",
        CatCount {
            counts: vec![
                29365,
                361,
                128,
                720,
            ],
            total: 30574,
        },
    ),
    (
        "2016-12",
        CatCount {
            counts: vec![
                23129,
                409,
                106,
                525,
            ],
            total: 24169,
        },
    ),
    (
        "2017-00",
        CatCount {
            counts: vec![
                6,
                0,
                0,
                0,
            ],
            total: 6,
        },
    ),
    (
        "2017-01",
        CatCount {
            counts: vec![
                18359,
                207,
                162,
                357,
            ],
            total: 19085,
        },
    ),
    (
        "2017-02",
        CatCount {
            counts: vec![
                21673,
                283,
                106,
                433,
            ],
            total: 22495,
        },
    ),
    (
        "2017-03",
        CatCount {
            counts: vec![
                27881,
                372,
                72,
                755,
            ],
            total: 29080,
        },
    ),
    (
        "2017-04",
        CatCount {
            counts: vec![
                26574,
                384,
                121,
                478,
            ],
            total: 27557,
        },
    ),
    (
        "2017-05",
        CatCount {
            counts: vec![
                25266,
                386,
                88,
                755,
            ],
            total: 26495,
        },
    ),
    (
        "2017-06",
        CatCount {
            counts: vec![
                25493,
                348,
                124,
                540,
            ],
            total: 26505,
        },
    ),
    (
        "2017-07",
        CatCount {
            counts: vec![
                19711,
                285,
                150,
                445,
            ],
            total: 20591,
        },
    ),
    (
        "2017-08",
        CatCount {
            counts: vec![
                18285,
                290,
                72,
                639,
            ],
            total: 19286,
        },
    ),
    (
        "2017-09",
        CatCount {
            counts: vec![
                28889,
                491,
                188,
                562,
            ],
            total: 30130,
        },
    ),
    (
        "2017-10",
        CatCount {
            counts: vec![
                29911,
                414,
                126,
                574,
            ],
            total: 31025,
        },
    ),
    (
        "2017-11",
        CatCount {
            counts: vec![
                28921,
                433,
                107,
                494,
            ],
            total: 29955,
        },
    ),
    (
        "2017-12",
        CatCount {
            counts: vec![
                23070,
                338,
                105,
                701,
            ],
            total: 24214,
        },
    ),
    (
        "2018-01",
        CatCount {
            counts: vec![
                17851,
                288,
                122,
                485,
            ],
            total: 18746,
        },
    ),
    (
        "2018-02",
        CatCount {
            counts: vec![
                21920,
                386,
                139,
                376,
            ],
            total: 22821,
        },
    ),
    (
        "2018-03",
        CatCount {
            counts: vec![
                26701,
                415,
                153,
                579,
            ],
            total: 27848,
        },
    ),
    (
        "2018-04",
        CatCount {
            counts: vec![
                27802,
                447,
                218,
                605,
            ],
            total: 29072,
        },
    ),
    (
        "2018-05",
        CatCount {
            counts: vec![
                25127,
                351,
                140,
                490,
            ],
            total: 26108,
        },
    ),
    (
        "2018-06",
        CatCount {
            counts: vec![
                25877,
                306,
                185,
                522,
            ],
            total: 26890,
        },
    ),
    (
        "2018-07",
        CatCount {
            counts: vec![
                20417,
                278,
                194,
                695,
            ],
            total: 21584,
        },
    ),
    (
        "2018-08",
        CatCount {
            counts: vec![
                21097,
                309,
                144,
                395,
            ],
            total: 21945,
        },
    ),
    (
        "2018-09",
        CatCount {
            counts: vec![
                27119,
                322,
                92,
                526,
            ],
            total: 28059,
        },
    ),
    (
        "2018-10",
        CatCount {
            counts: vec![
                30444,
                415,
                154,
                448,
            ],
            total: 31461,
        },
    ),
    (
        "2018-11",
        CatCount {
            counts: vec![
                32624,
                493,
                91,
                486,
            ],
            total: 33694,
        },
    ),
    (
        "2018-12",
        CatCount {
            counts: vec![
                21040,
                336,
                101,
                597,
            ],
            total: 22074,
        },
    ),
    (
        "2019-01",
        CatCount {
            counts: vec![
                17481,
                322,
                84,
                353,
            ],
            total: 18240,
        },
    ),
    (
        "2019-02",
        CatCount {
            counts: vec![
                20803,
                339,
                75,
                274,
            ],
            total: 21491,
        },
    ),
    (
        "2019-03",
        CatCount {
            counts: vec![
                27798,
                427,
                96,
                520,
            ],
            total: 28841,
        },
    ),
    (
        "2019-04",
        CatCount {
            counts: vec![
                26245,
                346,
                139,
                370,
            ],
            total: 27100,
        },
    ),
    (
        "2019-05",
        CatCount {
            counts: vec![
                25400,
                454,
                152,
                427,
            ],
            total: 26433,
        },
    ),
    (
        "2019-06",
        CatCount {
            counts: vec![
                24072,
                424,
                258,
                375,
            ],
            total: 25129,
        },
    ),
    (
        "2019-07",
        CatCount {
            counts: vec![
                18782,
                256,
                98,
                452,
            ],
            total: 19588,
        },
    ),
    (
        "2019-08",
        CatCount {
            counts: vec![
                20064,
                350,
                120,
                395,
            ],
            total: 20929,
        },
    ),
    (
        "2019-09",
        CatCount {
            counts: vec![
                24939,
                399,
                174,
                509,
            ],
            total: 26021,
        },
    ),
    (
        "2019-10",
        CatCount {
            counts: vec![
                28795,
                427,
                123,
                518,
            ],
            total: 29863,
        },
    ),
    (
        "2019-11",
        CatCount {
            counts: vec![
                32904,
                426,
                114,
                575,
            ],
            total: 34019,
        },
    ),
    (
        "2019-12",
        CatCount {
            counts: vec![
                20558,
                390,
                122,
                560,
            ],
            total: 21630,
        },
    ),
    (
        "2020-01",
        CatCount {
            counts: vec![
                19137,
                322,
                145,
                346,
            ],
            total: 19950,
        },
    ),
    (
        "2020-02",
        CatCount {
            counts: vec![
                22414,
                381,
                111,
                380,
            ],
            total: 23286,
        },
    ),
    (
        "2020-03",
        CatCount {
            counts: vec![
                26595,
                603,
                156,
                536,
            ],
            total: 27890,
        },
    ),
    (
        "2020-04",
        CatCount {
            counts: vec![
                21420,
                280,
                140,
                379,
            ],
            total: 22219,
        },
    ),
    (
        "2020-05",
        CatCount {
            counts: vec![
                24074,
                477,
                173,
                333,
            ],
            total: 25057,
        },
    ),
    (
        "2020-06",
        CatCount {
            counts: vec![
                23157,
                375,
                188,
                355,
            ],
            total: 24075,
        },
    ),
    (
        "2020-07",
        CatCount {
            counts: vec![
                21172,
                363,
                166,
                541,
            ],
            total: 22242,
        },
    ),
    (
        "2020-08",
        CatCount {
            counts: vec![
                20000,
                341,
                157,
                397,
            ],
            total: 20895,
        },
    ),
    (
        "2020-09",
        CatCount {
            counts: vec![
                25464,
                394,
                167,
                492,
            ],
            total: 26517,
        },
    ),
    (
        "2020-10",
        CatCount {
            counts: vec![
                31156,
                451,
                131,
                697,
            ],
            total: 32435,
        },
    ),
    (
        "2020-11",
        CatCount {
            counts: vec![
                27442,
                394,
                144,
                643,
            ],
            total: 28623,
        },
    ),
    (
        "2020-12",
        CatCount {
            counts: vec![
                25372,
                415,
                184,
                653,
            ],
            total: 26624,
        },
    ),
    (
        "2021-01",
        CatCount {
            counts: vec![
                17993,
                262,
                87,
                248,
            ],
            total: 18590,
        },
    ),
    (
        "2021-02",
        CatCount {
            counts: vec![
                18978,
                394,
                94,
                372,
            ],
            total: 19838,
        },
    ),
    (
        "2021-03",
        CatCount {
            counts: vec![
                22613,
                466,
                159,
                376,
            ],
            total: 23614,
        },
    ),
    (
        "2021-04",
        CatCount {
            counts: vec![
                23249,
                394,
                151,
                320,
            ],
            total: 24114,
        },
    ),
    (
        "2021-05",
        CatCount {
            counts: vec![
                21726,
                368,
                131,
                370,
            ],
            total: 22595,
        },
    ),
    (
        "2021-06",
        CatCount {
            counts: vec![
                20941,
                296,
                111,
                396,
            ],
            total: 21744,
        },
    ),
    (
        "2021-07",
        CatCount {
            counts: vec![
                18958,
                316,
                147,
                439,
            ],
            total: 19860,
        },
    ),
    (
        "2021-08",
        CatCount {
            counts: vec![
                16652,
                279,
                100,
                432,
            ],
            total: 17463,
        },
    ),
    (
        "2021-09",
        CatCount {
            counts: vec![
                21892,
                313,
                144,
                325,
            ],
            total: 22674,
        },
    ),
    (
        "2021-10",
        CatCount {
            counts: vec![
                24841,
                345,
                215,
                485,
            ],
            total: 25886,
        },
    ),
    (
        "2021-11",
        CatCount {
            counts: vec![
                23096,
                284,
                147,
                406,
            ],
            total: 23933,
        },
    ),
    (
        "2021-12",
        CatCount {
            counts: vec![
                19345,
                352,
                153,
                461,
            ],
            total: 20311,
        },
    ),
    (
        "2022-01",
        CatCount {
            counts: vec![
                13660,
                228,
                140,
                291,
            ],
            total: 14319,
        },
    ),
    (
        "2022-02",
        CatCount {
            counts: vec![
                16975,
                309,
                100,
                308,
            ],
            total: 17692,
        },
    ),
    (
        "2022-03",
        CatCount {
            counts: vec![
                18842,
                286,
                97,
                269,
            ],
            total: 19494,
        },
    ),
    (
        "2022-04",
        CatCount {
            counts: vec![
                19731,
                318,
                175,
                280,
            ],
            total: 20504,
        },
    ),
    (
        "2022-05",
        CatCount {
            counts: vec![
                17485,
                231,
                175,
                238,
            ],
            total: 18129,
        },
    ),
    (
        "2022-06",
        CatCount {
            counts: vec![
                15946,
                186,
                81,
                259,
            ],
            total: 16472,
        },
    ),
    (
        "2022-07",
        CatCount {
            counts: vec![
                14068,
                223,
                126,
                228,
            ],
            total: 14645,
        },
    ),
    (
        "2022-08",
        CatCount {
            counts: vec![
                12311,
                185,
                78,
                262,
            ],
            total: 12836,
        },
    ),
    (
        "2022-09",
        CatCount {
            counts: vec![
                17478,
                233,
                101,
                200,
            ],
            total: 18012,
        },
    ),
    (
        "2022-10",
        CatCount {
            counts: vec![
                15736,
                211,
                86,
                138,
            ],
            total: 16171,
        },
    ),
    (
        "2022-11",
        CatCount {
            counts: vec![
                13524,
                174,
                72,
                185,
            ],
            total: 13955,
        },
    ),
    (
        "2022-12",
        CatCount {
            counts: vec![
                7906,
                124,
                62,
                85,
            ],
            total: 8177,
        },
    ),
    (
        "2023-01",
        CatCount {
            counts: vec![
                117,
                0,
                0,
                0,
            ],
            total: 117,
        },
    ),
    (
        "2023-02",
        CatCount {
            counts: vec![
                27,
                0,
                0,
                0,
            ],
            total: 27,
        },
    ),
    (
        "2023-03",
        CatCount {
            counts: vec![
                13,
                0,
                0,
                0,
            ],
            total: 13,
        },
    ),
    (
        "2023-04",
        CatCount {
            counts: vec![
                21,
                0,
                0,
                0,
            ],
            total: 21,
        },
    ),
    ]
}

fn print_fracs() {
    let data = thing();
    for (k,v) in data {
        if k.ends_with("-00") {
            continue;
        }
        let lowercase = v.counts[data::stats::TitleType::LowercaseOnly as usize];
        let total = v.total;
        println!("{}\t{}\t{}", k, lowercase, total);
    }
}

fn main() {
    print_fracs()
}
