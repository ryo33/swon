// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans};
use parol_runtime::{ParolError, ParseTree, TerminalIndex};
use parol_runtime::{ScannerConfig, TokenStream, Tokenizer};
use std::path::Path;

use crate::grammar::Grammar;
use crate::grammar_trait::GrammarAuto;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 29] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r":",
    /*  6 */ r"todo",
    /*  7 */ r"@",
    /*  8 */ r"\.",
    /*  9 */ r"\p{XID_Start}\p{XID_Continue}*",
    /* 10 */ r"\$",
    /* 11 */ r"\d[\d_]*",
    /* 12 */ r"true",
    /* 13 */ r"false",
    /* 14 */ r"null",
    /* 15 */ r#"""#,
    /* 16 */ r#"\\[\\nrt\"0]"#,
    /* 17 */ r"\\u[0-9a-fA-F]{4}",
    /* 18 */ r"\\U[0-9a-fA-F]{8}",
    /* 19 */ r"\p{Letter}\p{Mark}\p{Number}\p{Punctuation}\p{Symbol}\p{Space_Separator}",
    /* 20 */ r"\r\n|\r|\n",
    /* 21 */ r"\{",
    /* 22 */ r"\}",
    /* 23 */ r"\[",
    /* 24 */ r"\]",
    /* 25 */ r"=",
    /* 26 */ r",",
    /* 27 */ r"\\\\",
    /* 28 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 29] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "Colon",
    /*  6 */ "Text",
    /*  7 */ "At",
    /*  8 */ "Dot",
    /*  9 */ "Ident",
    /* 10 */ "Dollar",
    /* 11 */ "Integer",
    /* 12 */ "True",
    /* 13 */ "False",
    /* 14 */ "Null",
    /* 15 */ "Quote",
    /* 16 */ "Escaped",
    /* 17 */ "Unicode4",
    /* 18 */ "Unicode8",
    /* 19 */ "Nonescaped",
    /* 20 */ "Newline0",
    /* 21 */ "Begin",
    /* 22 */ "End",
    /* 23 */ "ArrayBegin",
    /* 24 */ "ArrayEnd",
    /* 25 */ "Bind",
    /* 26 */ "Comma",
    /* 27 */ "Continue",
    /* 28 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[TerminalIndex; 19]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ UNMATCHABLE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ r"(#.*(\r\n|\r|\n|$))",
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5,  /* Colon */
        6,  /* Text */
        7,  /* At */
        8,  /* Dot */
        9,  /* Ident */
        10, /* Dollar */
        11, /* Integer */
        12, /* True */
        13, /* False */
        14, /* Null */
        15, /* Quote */
        20, /* Newline0 */
        21, /* Begin */
        22, /* End */
        23, /* ArrayBegin */
        24, /* ArrayEnd */
        25, /* Bind */
        26, /* Comma */
        27, /* Continue */
    ],
);

/* SCANNER_1: "String" */
const SCANNER_1: (&[&str; 5], &[TerminalIndex; 5]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ UNMATCHABLE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        15, /* Quote */
        16, /* Escaped */
        17, /* Unicode4 */
        18, /* Unicode8 */
        19, /* Nonescaped */
    ],
);

/* SCANNER_2: "ValueContext" */
const SCANNER_2: (&[&str; 5], &[TerminalIndex; 11]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        11, /* Integer */
        12, /* True */
        13, /* False */
        14, /* Null */
        21, /* Begin */
        22, /* End */
        23, /* ArrayBegin */
        24, /* ArrayEnd */
        25, /* Bind */
        26, /* Comma */
        27, /* Continue */
    ],
);

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 49] = &[
    /*  0 */ "Array",
    /*  1 */ "ArrayBegin",
    /*  2 */ "ArrayEnd",
    /*  3 */ "ArrayList",
    /*  4 */ "ArrayMarker",
    /*  5 */ "ArrayMarkerOpt",
    /*  6 */ "Begin",
    /*  7 */ "Bind",
    /*  8 */ "Binding",
    /*  9 */ "Bindings",
    /* 10 */ "Boolean",
    /* 11 */ "Char",
    /* 12 */ "Comma",
    /* 13 */ "Continue",
    /* 14 */ "End",
    /* 15 */ "Escaped",
    /* 16 */ "ExtensionNameSpace",
    /* 17 */ "False",
    /* 18 */ "Ident",
    /* 19 */ "Integer",
    /* 20 */ "Key",
    /* 21 */ "KeyBase",
    /* 22 */ "KeyOpt",
    /* 23 */ "Keys",
    /* 24 */ "KeysList",
    /* 25 */ "Newline",
    /* 26 */ "Nonescaped",
    /* 27 */ "Null",
    /* 28 */ "Object",
    /* 29 */ "ObjectList",
    /* 30 */ "Quote",
    /* 31 */ "Section",
    /* 32 */ "SectionBinding",
    /* 33 */ "SectionHeader",
    /* 34 */ "String",
    /* 35 */ "StringList",
    /* 36 */ "StringOpt",
    /* 37 */ "Swon",
    /* 38 */ "SwonList",
    /* 39 */ "SwonList0",
    /* 40 */ "Text",
    /* 41 */ "TextBinding",
    /* 42 */ "TextBindingOpt",
    /* 43 */ "True",
    /* 44 */ "Unicode4",
    /* 45 */ "Unicode8",
    /* 46 */ "Value",
    /* 47 */ "ValueBinding",
    /* 48 */ "ValueBindingOpt",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 49] = &[
    /* 0 - "Array" */
    LookaheadDFA {
        prod0: 42,
        transitions: &[],
        k: 0,
    },
    /* 1 - "ArrayBegin" */
    LookaheadDFA {
        prod0: 68,
        transitions: &[],
        k: 0,
    },
    /* 2 - "ArrayEnd" */
    LookaheadDFA {
        prod0: 69,
        transitions: &[],
        k: 0,
    },
    /* 3 - "ArrayList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 11, 1, 43),
            Trans(0, 12, 1, 43),
            Trans(0, 13, 1, 43),
            Trans(0, 14, 1, 43),
            Trans(0, 15, 1, 43),
            Trans(0, 21, 1, 43),
            Trans(0, 23, 1, 43),
            Trans(0, 24, 2, 44),
        ],
        k: 1,
    },
    /* 4 - "ArrayMarker" */
    LookaheadDFA {
        prod0: 25,
        transitions: &[],
        k: 0,
    },
    /* 5 - "ArrayMarkerOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 11, 1, 26), Trans(0, 24, 2, 27)],
        k: 1,
    },
    /* 6 - "Begin" */
    LookaheadDFA {
        prod0: 66,
        transitions: &[],
        k: 0,
    },
    /* 7 - "Bind" */
    LookaheadDFA {
        prod0: 70,
        transitions: &[],
        k: 0,
    },
    /* 8 - "Binding" */
    LookaheadDFA {
        prod0: 6,
        transitions: &[],
        k: 0,
    },
    /* 9 - "Bindings" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 5, 3, 9), Trans(0, 21, 2, 8), Trans(0, 25, 1, 7)],
        k: 1,
    },
    /* 10 - "Boolean" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 12, 1, 46), Trans(0, 13, 2, 47)],
        k: 1,
    },
    /* 11 - "Char" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 16, 1, 56),
            Trans(0, 17, 2, 57),
            Trans(0, 18, 3, 58),
            Trans(0, 19, 4, 59),
        ],
        k: 1,
    },
    /* 12 - "Comma" */
    LookaheadDFA {
        prod0: 71,
        transitions: &[],
        k: 0,
    },
    /* 13 - "Continue" */
    LookaheadDFA {
        prod0: 72,
        transitions: &[],
        k: 0,
    },
    /* 14 - "End" */
    LookaheadDFA {
        prod0: 67,
        transitions: &[],
        k: 0,
    },
    /* 15 - "Escaped" */
    LookaheadDFA {
        prod0: 61,
        transitions: &[],
        k: 0,
    },
    /* 16 - "ExtensionNameSpace" */
    LookaheadDFA {
        prod0: 32,
        transitions: &[],
        k: 0,
    },
    /* 17 - "False" */
    LookaheadDFA {
        prod0: 49,
        transitions: &[],
        k: 0,
    },
    /* 18 - "Ident" */
    LookaheadDFA {
        prod0: 31,
        transitions: &[],
        k: 0,
    },
    /* 19 - "Integer" */
    LookaheadDFA {
        prod0: 45,
        transitions: &[],
        k: 0,
    },
    /* 20 - "Key" */
    LookaheadDFA {
        prod0: 22,
        transitions: &[],
        k: 0,
    },
    /* 21 - "KeyBase" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 9, 1, 28), Trans(0, 10, 2, 29), Trans(0, 15, 3, 30)],
        k: 1,
    },
    /* 22 - "KeyOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 2, 24),
            Trans(0, 8, 2, 24),
            Trans(0, 20, 2, 24),
            Trans(0, 21, 2, 24),
            Trans(0, 23, 1, 23),
            Trans(0, 25, 2, 24),
        ],
        k: 1,
    },
    /* 23 - "Keys" */
    LookaheadDFA {
        prod0: 19,
        transitions: &[],
        k: 0,
    },
    /* 24 - "KeysList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 2, 21),
            Trans(0, 8, 1, 20),
            Trans(0, 20, 2, 21),
            Trans(0, 21, 2, 21),
            Trans(0, 25, 2, 21),
        ],
        k: 1,
    },
    /* 25 - "Newline" */
    LookaheadDFA {
        prod0: 65,
        transitions: &[],
        k: 0,
    },
    /* 26 - "Nonescaped" */
    LookaheadDFA {
        prod0: 64,
        transitions: &[],
        k: 0,
    },
    /* 27 - "Null" */
    LookaheadDFA {
        prod0: 50,
        transitions: &[],
        k: 0,
    },
    /* 28 - "Object" */
    LookaheadDFA {
        prod0: 39,
        transitions: &[],
        k: 0,
    },
    /* 29 - "ObjectList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 9, 1, 40),
            Trans(0, 10, 1, 40),
            Trans(0, 15, 1, 40),
            Trans(0, 22, 2, 41),
        ],
        k: 1,
    },
    /* 30 - "Quote" */
    LookaheadDFA {
        prod0: 60,
        transitions: &[],
        k: 0,
    },
    /* 31 - "Section" */
    LookaheadDFA {
        prod0: 5,
        transitions: &[],
        k: 0,
    },
    /* 32 - "SectionBinding" */
    LookaheadDFA {
        prod0: 13,
        transitions: &[],
        k: 0,
    },
    /* 33 - "SectionHeader" */
    LookaheadDFA {
        prod0: 18,
        transitions: &[],
        k: 0,
    },
    /* 34 - "String" */
    LookaheadDFA {
        prod0: 51,
        transitions: &[],
        k: 0,
    },
    /* 35 - "StringList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 15, 2, 53),
            Trans(0, 16, 1, 52),
            Trans(0, 17, 1, 52),
            Trans(0, 18, 1, 52),
            Trans(0, 19, 1, 52),
        ],
        k: 1,
    },
    /* 36 - "StringOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 2, 55),
            Trans(0, 8, 2, 55),
            Trans(0, 20, 2, 55),
            Trans(0, 21, 2, 55),
            Trans(0, 23, 2, 55),
            Trans(0, 25, 2, 55),
            Trans(0, 26, 2, 55),
            Trans(0, 27, 1, 54),
        ],
        k: 1,
    },
    /* 37 - "Swon" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 38 - "SwonList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 4),
            Trans(0, 7, 2, 4),
            Trans(0, 9, 1, 3),
            Trans(0, 10, 1, 3),
            Trans(0, 15, 1, 3),
            Trans(0, 20, 2, 4),
        ],
        k: 1,
    },
    /* 39 - "SwonList0" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 0, 2, 2), Trans(0, 7, 1, 1), Trans(0, 20, 2, 2)],
        k: 1,
    },
    /* 40 - "Text" */
    LookaheadDFA {
        prod0: 17,
        transitions: &[],
        k: 0,
    },
    /* 41 - "TextBinding" */
    LookaheadDFA {
        prod0: 14,
        transitions: &[],
        k: 0,
    },
    /* 42 - "TextBindingOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 6, 2, 16), Trans(0, 20, 1, 15)],
        k: 1,
    },
    /* 43 - "True" */
    LookaheadDFA {
        prod0: 48,
        transitions: &[],
        k: 0,
    },
    /* 44 - "Unicode4" */
    LookaheadDFA {
        prod0: 62,
        transitions: &[],
        k: 0,
    },
    /* 45 - "Unicode8" */
    LookaheadDFA {
        prod0: 63,
        transitions: &[],
        k: 0,
    },
    /* 46 - "Value" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 11, 3, 35),
            Trans(0, 12, 4, 36),
            Trans(0, 13, 4, 36),
            Trans(0, 14, 5, 37),
            Trans(0, 15, 6, 38),
            Trans(0, 21, 1, 33),
            Trans(0, 23, 2, 34),
        ],
        k: 1,
    },
    /* 47 - "ValueBinding" */
    LookaheadDFA {
        prod0: 10,
        transitions: &[],
        k: 0,
    },
    /* 48 - "ValueBindingOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 11, 2, 12),
            Trans(0, 12, 2, 12),
            Trans(0, 13, 2, 12),
            Trans(0, 14, 2, 12),
            Trans(0, 15, 2, 12),
            Trans(0, 20, 1, 11),
            Trans(0, 21, 2, 12),
            Trans(0, 23, 2, 12),
        ],
        k: 1,
    },
];

pub const PRODUCTIONS: &[Production; 73] = &[
    // 0 - Swon: SwonList /* Vec */ SwonList0 /* Vec */;
    Production {
        lhs: 37,
        production: &[ParseType::N(39), ParseType::N(38)],
    },
    // 1 - SwonList0: Section SwonList0;
    Production {
        lhs: 39,
        production: &[ParseType::N(39), ParseType::N(31)],
    },
    // 2 - SwonList0: ;
    Production {
        lhs: 39,
        production: &[],
    },
    // 3 - SwonList: Binding SwonList;
    Production {
        lhs: 38,
        production: &[ParseType::N(38), ParseType::N(8)],
    },
    // 4 - SwonList: ;
    Production {
        lhs: 38,
        production: &[],
    },
    // 5 - Section: SectionHeader Swon Newline;
    Production {
        lhs: 31,
        production: &[ParseType::N(25), ParseType::N(37), ParseType::N(33)],
    },
    // 6 - Binding: Keys Bindings Newline;
    Production {
        lhs: 8,
        production: &[ParseType::N(25), ParseType::N(9), ParseType::N(23)],
    },
    // 7 - Bindings: ValueBinding;
    Production {
        lhs: 9,
        production: &[ParseType::N(47)],
    },
    // 8 - Bindings: SectionBinding;
    Production {
        lhs: 9,
        production: &[ParseType::N(32)],
    },
    // 9 - Bindings: TextBinding;
    Production {
        lhs: 9,
        production: &[ParseType::N(41)],
    },
    // 10 - ValueBinding: Bind^ /* Clipped */ ValueBindingOpt /* Option */ Push(2) Value Pop;
    Production {
        lhs: 47,
        production: &[
            ParseType::Pop,
            ParseType::N(46),
            ParseType::Push(2),
            ParseType::N(48),
            ParseType::N(7),
        ],
    },
    // 11 - ValueBindingOpt: Newline;
    Production {
        lhs: 48,
        production: &[ParseType::N(25)],
    },
    // 12 - ValueBindingOpt: ;
    Production {
        lhs: 48,
        production: &[],
    },
    // 13 - SectionBinding: Begin^ /* Clipped */ Newline Swon Newline End^ /* Clipped */;
    Production {
        lhs: 32,
        production: &[
            ParseType::N(14),
            ParseType::N(25),
            ParseType::N(37),
            ParseType::N(25),
            ParseType::N(6),
        ],
    },
    // 14 - TextBinding: ':'^ /* Clipped */ TextBindingOpt /* Option */ Text;
    Production {
        lhs: 41,
        production: &[ParseType::N(40), ParseType::N(42), ParseType::T(5)],
    },
    // 15 - TextBindingOpt: Newline;
    Production {
        lhs: 42,
        production: &[ParseType::N(25)],
    },
    // 16 - TextBindingOpt: ;
    Production {
        lhs: 42,
        production: &[],
    },
    // 17 - Text: /todo/;
    Production {
        lhs: 40,
        production: &[ParseType::T(6)],
    },
    // 18 - SectionHeader: '@'^ /* Clipped */ Keys Newline;
    Production {
        lhs: 33,
        production: &[ParseType::N(25), ParseType::N(23), ParseType::T(7)],
    },
    // 19 - Keys: Key KeysList /* Vec */;
    Production {
        lhs: 23,
        production: &[ParseType::N(24), ParseType::N(20)],
    },
    // 20 - KeysList: '.'^ /* Clipped */ Key KeysList;
    Production {
        lhs: 24,
        production: &[ParseType::N(24), ParseType::N(20), ParseType::T(8)],
    },
    // 21 - KeysList: ;
    Production {
        lhs: 24,
        production: &[],
    },
    // 22 - Key: KeyBase KeyOpt /* Option */;
    Production {
        lhs: 20,
        production: &[ParseType::N(22), ParseType::N(21)],
    },
    // 23 - KeyOpt: ArrayMarker;
    Production {
        lhs: 22,
        production: &[ParseType::N(4)],
    },
    // 24 - KeyOpt: ;
    Production {
        lhs: 22,
        production: &[],
    },
    // 25 - ArrayMarker: ArrayBegin^ /* Clipped */ ArrayMarkerOpt /* Option */ ArrayEnd^ /* Clipped */;
    Production {
        lhs: 4,
        production: &[ParseType::N(2), ParseType::N(5), ParseType::N(1)],
    },
    // 26 - ArrayMarkerOpt: Integer;
    Production {
        lhs: 5,
        production: &[ParseType::N(19)],
    },
    // 27 - ArrayMarkerOpt: ;
    Production {
        lhs: 5,
        production: &[],
    },
    // 28 - KeyBase: Ident;
    Production {
        lhs: 21,
        production: &[ParseType::N(18)],
    },
    // 29 - KeyBase: ExtensionNameSpace;
    Production {
        lhs: 21,
        production: &[ParseType::N(16)],
    },
    // 30 - KeyBase: String;
    Production {
        lhs: 21,
        production: &[ParseType::N(34)],
    },
    // 31 - Ident: /\p{XID_Start}\p{XID_Continue}*/;
    Production {
        lhs: 18,
        production: &[ParseType::T(9)],
    },
    // 32 - ExtensionNameSpace: '$'^ /* Clipped */ Ident;
    Production {
        lhs: 16,
        production: &[ParseType::N(18), ParseType::T(10)],
    },
    // 33 - Value: Object;
    Production {
        lhs: 46,
        production: &[ParseType::N(28)],
    },
    // 34 - Value: Array;
    Production {
        lhs: 46,
        production: &[ParseType::N(0)],
    },
    // 35 - Value: Integer;
    Production {
        lhs: 46,
        production: &[ParseType::N(19)],
    },
    // 36 - Value: Boolean;
    Production {
        lhs: 46,
        production: &[ParseType::N(10)],
    },
    // 37 - Value: Null;
    Production {
        lhs: 46,
        production: &[ParseType::N(27)],
    },
    // 38 - Value: String;
    Production {
        lhs: 46,
        production: &[ParseType::N(34)],
    },
    // 39 - Object: Begin^ /* Clipped */ ObjectList /* Vec */ End^ /* Clipped */;
    Production {
        lhs: 28,
        production: &[ParseType::N(14), ParseType::N(29), ParseType::N(6)],
    },
    // 40 - ObjectList: Key Bind^ /* Clipped */ Value Comma^ /* Clipped */ ObjectList;
    Production {
        lhs: 29,
        production: &[
            ParseType::N(29),
            ParseType::N(12),
            ParseType::N(46),
            ParseType::N(7),
            ParseType::N(20),
        ],
    },
    // 41 - ObjectList: ;
    Production {
        lhs: 29,
        production: &[],
    },
    // 42 - Array: ArrayBegin^ /* Clipped */ ArrayList /* Vec */ ArrayEnd^ /* Clipped */;
    Production {
        lhs: 0,
        production: &[ParseType::N(2), ParseType::N(3), ParseType::N(1)],
    },
    // 43 - ArrayList: Value Comma^ /* Clipped */ ArrayList;
    Production {
        lhs: 3,
        production: &[ParseType::N(3), ParseType::N(12), ParseType::N(46)],
    },
    // 44 - ArrayList: ;
    Production {
        lhs: 3,
        production: &[],
    },
    // 45 - Integer: /\d[\d_]*/;
    Production {
        lhs: 19,
        production: &[ParseType::T(11)],
    },
    // 46 - Boolean: True;
    Production {
        lhs: 10,
        production: &[ParseType::N(43)],
    },
    // 47 - Boolean: False;
    Production {
        lhs: 10,
        production: &[ParseType::N(17)],
    },
    // 48 - True: 'true';
    Production {
        lhs: 43,
        production: &[ParseType::T(12)],
    },
    // 49 - False: 'false';
    Production {
        lhs: 17,
        production: &[ParseType::T(13)],
    },
    // 50 - Null: 'null';
    Production {
        lhs: 27,
        production: &[ParseType::T(14)],
    },
    // 51 - String: Quote^ /* Clipped */ Push(1) StringList /* Vec */ Quote^ /* Clipped */ Pop StringOpt /* Option */;
    Production {
        lhs: 34,
        production: &[
            ParseType::N(36),
            ParseType::Pop,
            ParseType::N(30),
            ParseType::N(35),
            ParseType::Push(1),
            ParseType::N(30),
        ],
    },
    // 52 - StringList: Char StringList;
    Production {
        lhs: 35,
        production: &[ParseType::N(35), ParseType::N(11)],
    },
    // 53 - StringList: ;
    Production {
        lhs: 35,
        production: &[],
    },
    // 54 - StringOpt: Continue^ /* Clipped */ String;
    Production {
        lhs: 36,
        production: &[ParseType::N(34), ParseType::N(13)],
    },
    // 55 - StringOpt: ;
    Production {
        lhs: 36,
        production: &[],
    },
    // 56 - Char: Escaped;
    Production {
        lhs: 11,
        production: &[ParseType::N(15)],
    },
    // 57 - Char: Unicode4;
    Production {
        lhs: 11,
        production: &[ParseType::N(44)],
    },
    // 58 - Char: Unicode8;
    Production {
        lhs: 11,
        production: &[ParseType::N(45)],
    },
    // 59 - Char: Nonescaped;
    Production {
        lhs: 11,
        production: &[ParseType::N(26)],
    },
    // 60 - Quote: /"/;
    Production {
        lhs: 30,
        production: &[ParseType::T(15)],
    },
    // 61 - Escaped: /\\[\\nrt\"0]/;
    Production {
        lhs: 15,
        production: &[ParseType::T(16)],
    },
    // 62 - Unicode4: /\\u[0-9a-fA-F]{4}/;
    Production {
        lhs: 44,
        production: &[ParseType::T(17)],
    },
    // 63 - Unicode8: /\\U[0-9a-fA-F]{8}/;
    Production {
        lhs: 45,
        production: &[ParseType::T(18)],
    },
    // 64 - Nonescaped: /\p{Letter}\p{Mark}\p{Number}\p{Punctuation}\p{Symbol}\p{Space_Separator}/;
    Production {
        lhs: 26,
        production: &[ParseType::T(19)],
    },
    // 65 - Newline: /\r\n|\r|\n/;
    Production {
        lhs: 25,
        production: &[ParseType::T(20)],
    },
    // 66 - Begin: '{';
    Production {
        lhs: 6,
        production: &[ParseType::T(21)],
    },
    // 67 - End: '}';
    Production {
        lhs: 14,
        production: &[ParseType::T(22)],
    },
    // 68 - ArrayBegin: '[';
    Production {
        lhs: 1,
        production: &[ParseType::T(23)],
    },
    // 69 - ArrayEnd: ']';
    Production {
        lhs: 2,
        production: &[ParseType::T(24)],
    },
    // 70 - Bind: '=';
    Production {
        lhs: 7,
        production: &[ParseType::T(25)],
    },
    // 71 - Comma: ',';
    Production {
        lhs: 12,
        production: &[ParseType::T(26)],
    },
    // 72 - Continue: '\\';
    Production {
        lhs: 13,
        production: &[ParseType::T(27)],
    },
];

static SCANNERS: Lazy<Vec<ScannerConfig>> = Lazy::new(|| {
    vec![
        ScannerConfig::new(
            "INITIAL",
            Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
            &[],
        ),
        ScannerConfig::new(
            "String",
            Tokenizer::build(TERMINALS, SCANNER_1.0, SCANNER_1.1).unwrap(),
            &[],
        ),
        ScannerConfig::new(
            "ValueContext",
            Tokenizer::build(TERMINALS, SCANNER_2.0, SCANNER_2.1).unwrap(),
            &[],
        ),
    ]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut Grammar<'t>,
) -> Result<ParseTree<'t>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        37,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    llk_parser.trim_parse_tree();

    // Initialize wrapper
    let mut user_actions = GrammarAuto::new(user_actions);
    llk_parser.parse(
        TokenStream::new(input, file_name, &SCANNERS, MAX_K).unwrap(),
        &mut user_actions,
    )
}
