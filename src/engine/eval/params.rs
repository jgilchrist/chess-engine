#![cfg_attr(any(), rustfmt::skip)]

use crate::chess::square::{File, Rank};
use crate::engine::eval::PhasedEval;

pub const fn s(mg: i16, eg: i16) -> PhasedEval {
    PhasedEval::new(mg, eg)
}

pub type PieceSquareTableDefinition = [[PhasedEval; File::N]; Rank::N];

pub const PIECE_VALUES: [PhasedEval; 6] = [
    s(  101,   169),
    s(  274,   329),
    s(  283,   345),
    s(  376,   603),
    s(  771,  1148),
    s(    0,     0),
];

pub const PAWNS: PieceSquareTableDefinition = [
    [s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0)],
    [s(    8,    86), s(   24,    79), s(    7,    78), s(   24,    41), s(   11,    41), s(    4,    52), s(  -36,    80), s(  -55,    91)],
    [s(  -10,    20), s(  -11,    26), s(   10,   -12), s(   16,     9), s(   34,   -29), s(   82,   -28), s(   78,    23), s(   30,     8)],
    [s(  -34,    13), s(  -13,     7), s(  -17,   -17), s(  -17,   -33), s(   11,   -33), s(    9,   -32), s(   13,    -6), s(   -2,   -19)],
    [s(  -39,    -8), s(  -18,     2), s(  -14,   -25), s(    0,   -28), s(    2,   -29), s(   -3,   -27), s(    2,   -14), s(  -17,   -32)],
    [s(  -40,   -12), s(  -11,    -7), s(  -18,   -26), s(  -10,   -16), s(   10,   -18), s(  -11,   -23), s(   31,   -20), s(   -8,   -34)],
    [s(  -30,    -9), s(   -6,    -2), s(  -11,   -23), s(  -11,   -10), s(    5,    -2), s(   26,   -20), s(   49,   -21), s(  -10,   -37)],
    [s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0)],
];

pub const KNIGHTS: PieceSquareTableDefinition = [
    [s( -136,   -49), s( -144,   -11), s(  -81,     4), s(  -31,    -7), s(    6,    -2), s(  -63,   -32), s( -119,    -5), s(  -76,   -77)],
    [s(   -4,    -6), s(    5,     3), s(   34,    -2), s(   47,    -2), s(   22,    -9), s(   95,   -29), s(   13,    -2), s(   39,   -24)],
    [s(    4,    -8), s(   36,    -5), s(   35,    16), s(   49,    19), s(   71,     7), s(   82,    -7), s(   39,    -8), s(   19,   -12)],
    [s(   12,    11), s(   12,    15), s(   31,    28), s(   69,    29), s(   34,    34), s(   59,    28), s(    5,    24), s(   53,     2)],
    [s(   -1,    18), s(    3,     6), s(    9,    28), s(   24,    29), s(   29,    35), s(   27,    18), s(   30,     6), s(   16,    11)],
    [s(  -27,    -4), s(  -14,    -4), s(  -10,     2), s(   -8,    22), s(   12,    19), s(   -8,    -5), s(   12,    -8), s(   -6,    -2)],
    [s(  -28,    -2), s(  -25,     1), s(  -17,   -11), s(    7,    -6), s(    2,    -7), s(    3,   -13), s(   -7,    -9), s(   -3,    15)],
    [s(  -81,    15), s(   -8,   -16), s(  -38,   -13), s(  -17,   -11), s(  -11,    -6), s(   -8,   -20), s(   -6,    -8), s(  -37,     1)],
];

pub const BISHOPS: PieceSquareTableDefinition = [
    [s(  -21,    -1), s(  -75,    14), s(  -61,     7), s( -125,    22), s( -100,    14), s(  -82,     0), s(  -48,     3), s(  -63,    -6)],
    [s(  -15,   -14), s(    1,    -9), s(  -12,    -5), s(  -27,     0), s(    0,   -12), s(  -16,    -6), s(  -23,     0), s(  -14,   -13)],
    [s(   -2,     8), s(   17,    -5), s(   15,     0), s(   24,   -10), s(    5,    -1), s(   49,     3), s(   22,    -1), s(   14,    11)],
    [s(  -11,     3), s(    8,     7), s(   14,     2), s(   36,    21), s(   26,     9), s(   21,    10), s(   10,     2), s(  -14,     8)],
    [s(   -1,    -2), s(  -14,     8), s(   -1,    14), s(   25,    12), s(   22,    11), s(    1,     5), s(   -7,     6), s(   22,   -16)],
    [s(    6,     1), s(   14,     6), s(    7,     6), s(   13,     9), s(   14,    13), s(   11,     6), s(   15,    -9), s(   30,    -6)],
    [s(   36,    12), s(   12,   -13), s(   26,   -17), s(    1,    -4), s(    9,    -1), s(   26,   -14), s(   36,    -5), s(   33,   -10)],
    [s(   14,    -4), s(   35,     2), s(   17,   -12), s(   -3,    -8), s(    6,   -11), s(    3,     3), s(   29,   -19), s(   42,   -22)],
];

pub const ROOKS: PieceSquareTableDefinition = [
    [s(    8,    14), s(  -19,    27), s(  -28,    42), s(  -38,    39), s(  -21,    31), s(   17,    20), s(   18,    16), s(   44,     7)],
    [s(   -2,    15), s(   -8,    31), s(   12,    37), s(   34,    25), s(   15,    26), s(   34,    14), s(   37,     4), s(   69,   -10)],
    [s(  -15,    15), s(   24,    13), s(   18,    17), s(   26,    14), s(   58,    -3), s(   58,    -9), s(  114,   -22), s(   71,   -25)],
    [s(  -23,    19), s(   -1,    13), s(    0,    22), s(    5,    17), s(   16,    -1), s(   15,    -8), s(   29,   -10), s(   25,   -17)],
    [s(  -41,    10), s(  -40,    12), s(  -29,    12), s(  -15,     8), s(  -15,     5), s(  -34,     4), s(    0,   -12), s(  -18,   -13)],
    [s(  -48,     2), s(  -38,    -3), s(  -28,    -6), s(  -26,    -2), s(  -17,    -9), s(  -20,   -19), s(   19,   -42), s(   -8,   -36)],
    [s(  -49,    -8), s(  -36,    -7), s(  -14,   -11), s(  -13,   -11), s(   -7,   -20), s(   -9,   -26), s(    8,   -35), s(  -26,   -25)],
    [s(  -26,   -10), s(  -19,   -10), s(   -3,    -5), s(    5,    -9), s(   12,   -19), s(   -3,   -16), s(    9,   -25), s(  -20,   -26)],
];

pub const QUEENS: PieceSquareTableDefinition = [
    [s(  -25,   -16), s(  -69,    25), s(  -56,    61), s(  -34,    51), s(  -51,    58), s(  -40,    52), s(   31,   -30), s(  -12,     0)],
    [s(    8,   -33), s(  -35,    13), s(  -40,    60), s(  -54,    84), s(  -73,   118), s(  -21,    60), s(  -29,    41), s(   53,    17)],
    [s(   11,   -23), s(   -1,    -8), s(  -10,    40), s(  -11,    56), s(   -9,    71), s(   17,    53), s(   36,    13), s(   16,    23)],
    [s(   -6,    -7), s(   -3,     9), s(  -15,    30), s(  -18,    57), s(  -17,    68), s(   -3,    53), s(   11,    49), s(   18,    23)],
    [s(    6,   -20), s(  -15,    18), s(  -12,    19), s(   -5,    40), s(   -5,    41), s(   -8,    33), s(   11,    17), s(   23,     6)],
    [s(    0,   -36), s(    6,   -19), s(   -5,     4), s(   -4,     5), s(    2,     8), s(    8,    -2), s(   24,   -25), s(   27,   -38)],
    [s(   18,   -52), s(    6,   -47), s(   17,   -48), s(   22,   -43), s(   18,   -36), s(   30,   -74), s(   38,  -111), s(   67,  -141)],
    [s(    8,   -57), s(   13,   -58), s(   22,   -62), s(   30,   -43), s(   26,   -65), s(    5,   -64), s(   32,   -92), s(   34,   -97)],
];

pub const KING: PieceSquareTableDefinition = [
    [s(   24,  -123), s(   18,   -51), s(   53,   -32), s( -119,    33), s(  -55,     9), s(    5,     9), s(   84,    -6), s(  184,  -148)],
    [s( -133,     9), s(  -30,    45), s(  -76,    61), s(   60,    41), s(   -1,    65), s(   -4,    83), s(   50,    65), s(   -9,    26)],
    [s( -151,    28), s(   25,    52), s(  -66,    80), s(  -95,    95), s(  -40,    95), s(   67,    82), s(   25,    80), s(  -31,    38)],
    [s( -108,    15), s(  -88,    59), s( -114,    87), s( -173,   105), s( -162,   105), s( -110,    97), s( -107,    81), s( -171,    48)],
    [s( -112,     4), s(  -85,    41), s( -120,    74), s( -167,    97), s( -151,    93), s( -105,    74), s( -109,    57), s( -194,    42)],
    [s(  -53,   -11), s(   -2,    20), s(  -69,    49), s(  -86,    65), s(  -72,    63), s(  -74,    51), s(  -21,    25), s(  -81,    14)],
    [s(   53,   -32), s(   15,     4), s(   -2,    20), s(  -48,    34), s(  -48,    38), s(  -27,    26), s(   34,     1), s(   27,   -23)],
    [s(   30,   -85), s(   75,   -55), s(   40,   -24), s(  -79,    -3), s(    0,   -27), s(  -45,    -7), s(   47,   -42), s(   37,   -87)],
];

pub const PASSED_PAWNS: PieceSquareTableDefinition = [
    [s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0)],
    [s(   43,   170), s(   59,   163), s(   42,   162), s(   59,   125), s(   46,   125), s(   39,   136), s(   -1,   164), s(  -20,   175)],
    [s(   18,   186), s(   41,   182), s(   27,   157), s(   15,    86), s(    4,   118), s(    3,   142), s(  -48,   151), s(  -67,   183)],
    [s(   13,    90), s(    4,    85), s(   24,    65), s(   21,    56), s(    2,    54), s(   11,    66), s(  -39,    88), s(  -29,    97)],
    [s(  -11,    48), s(  -18,    35), s(  -31,    34), s(  -18,    24), s(  -27,    28), s(  -14,    32), s(  -31,    50), s(  -25,    49)],
    [s(  -16,    -5), s(  -33,     6), s(  -35,    11), s(  -31,    -1), s(  -29,     2), s(  -14,     2), s(  -33,    25), s(   -7,     3)],
    [s(  -28,    -5), s(  -19,    -4), s(  -31,     7), s(  -29,     1), s(  -14,   -18), s(  -12,    -5), s(    1,    -3), s(  -20,     4)],
    [s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0)],
];

pub const KNIGHT_MOBILITY: [PhasedEval; 9] = [
    s(  -89,   -19),
    s(   55,    79),
    s(   87,   122),
    s(  101,   155),
    s(  117,   168),
    s(  119,   184),
    s(  134,   188),
    s(  149,   192),
    s(  165,   185),
];

pub const BISHOP_MOBILITY: [PhasedEval; 14] = [
    s(   58,   -94),
    s(   34,    29),
    s(   84,   101),
    s(   99,   137),
    s(  117,   152),
    s(  129,   163),
    s(  138,   177),
    s(  145,   184),
    s(  149,   193),
    s(  153,   194),
    s(  156,   198),
    s(  169,   189),
    s(  173,   193),
    s(  175,   181),
];

pub const ROOK_MOBILITY: [PhasedEval; 15] = [
    s(   79,    78),
    s(  105,   194),
    s(  135,   255),
    s(  147,   274),
    s(  157,   285),
    s(  163,   294),
    s(  168,   301),
    s(  174,   308),
    s(  182,   309),
    s(  193,   313),
    s(  203,   318),
    s(  211,   324),
    s(  218,   330),
    s(  222,   330),
    s(  215,   330),
];

pub const QUEEN_MOBILITY: [PhasedEval; 28] = [
    s(    0,     0),
    s(    0,     0),
    s(  -46,   -17),
    s(  326,   123),
    s(  292,   404),
    s(  328,   450),
    s(  338,   473),
    s(  346,   491),
    s(  354,   512),
    s(  359,   546),
    s(  366,   552),
    s(  373,   560),
    s(  378,   571),
    s(  385,   572),
    s(  387,   578),
    s(  389,   588),
    s(  390,   592),
    s(  388,   605),
    s(  388,   612),
    s(  388,   615),
    s(  397,   618),
    s(  408,   603),
    s(  427,   601),
    s(  447,   584),
    s(  450,   592),
    s(  609,   511),
    s(  510,   550),
    s(  425,   578),
];

pub const ATTACKED_KING_SQUARES: [PhasedEval; 9] = [
    s(   62,   -25),
    s(   55,   -19),
    s(   26,    -6),
    s(  -26,    -9),
    s( -115,    22),
    s( -229,    79),
    s( -346,   135),
    s( -512,   193),
    s( -277,   -81),
];

pub const BISHOP_PAIR_BONUS: PhasedEval = s(   31,    89);
