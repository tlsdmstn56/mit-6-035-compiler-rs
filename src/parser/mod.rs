#[cfg(test)]
mod tests {
    use crate::decaf::TProgramParser;
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use std::env;
    
macro_rules! test_parser_illegal {
    ( $testname:ident, $filename:expr ) => {
        #[test]
        fn $testname()
        {
            let path = env::var("CARGO_MANIFEST_DIR").unwrap();
            let path: PathBuf = [&path, "src", "parser", "testcases", "illegal", $filename].iter().collect();
            let s = read_to_string(&path).unwrap();
            assert!(TProgramParser::new().parse(&s).is_err());
        }
    };
}

macro_rules! test_parser_legal {
    ( $testname:ident, $filename:expr ) => {
        #[test]
        fn $testname()
        {
            let path = env::var("CARGO_MANIFEST_DIR").unwrap();
            let path: PathBuf = [&path, "src", "parser", "testcases", "legal", $filename].iter().collect();
            let s = read_to_string(&path).unwrap();
            let program = TProgramParser::new().parse(&s);
            program.unwrap();
            // assert!(program.is_ok());
        }
    };
}

    
    fn test_empty() {
        assert!(TProgramParser::new().parse("").is_err());
    }
    
    test_parser_legal!(test_legal_01, "legal-01");
    test_parser_legal!(test_legal_02, "legal-02");
    test_parser_legal!(test_legal_03, "legal-03");
    test_parser_legal!(test_legal_04, "legal-04");
    test_parser_legal!(test_legal_05, "legal-05");
    test_parser_legal!(test_legal_06, "legal-06");
    test_parser_legal!(test_legal_07, "legal-07");
    test_parser_legal!(test_legal_08, "legal-08");
    test_parser_legal!(test_legal_09, "legal-09");
    test_parser_legal!(test_legal_10, "legal-10");
    test_parser_legal!(test_legal_11, "legal-11");
    test_parser_legal!(test_legal_12, "legal-12");
    test_parser_legal!(test_legal_13, "legal-13");
    test_parser_legal!(test_legal_14, "legal-14");
    test_parser_legal!(test_legal_15, "legal-15");
    test_parser_legal!(test_legal_16, "legal-16");
    test_parser_legal!(test_legal_17, "legal-17");
    test_parser_legal!(test_legal_18, "legal-18");

    test_parser_illegal!(test_illegal_01, "illegal-01");
    test_parser_illegal!(test_illegal_02, "illegal-02");
    test_parser_illegal!(test_illegal_03, "illegal-03");
    test_parser_illegal!(test_illegal_04, "illegal-04");
    test_parser_illegal!(test_illegal_05, "illegal-05");
    test_parser_illegal!(test_illegal_06, "illegal-06");
    test_parser_illegal!(test_illegal_07, "illegal-07");
    test_parser_illegal!(test_illegal_08, "illegal-08");
    test_parser_illegal!(test_illegal_09, "illegal-09");
    test_parser_illegal!(test_illegal_10, "illegal-10");
    test_parser_illegal!(test_illegal_11, "illegal-11");
    test_parser_illegal!(test_illegal_12, "illegal-12");
    test_parser_illegal!(test_illegal_13, "illegal-13");
    test_parser_illegal!(test_illegal_14, "illegal-14");
    test_parser_illegal!(test_illegal_15, "illegal-15");
    test_parser_illegal!(test_illegal_16, "illegal-16");
    test_parser_illegal!(test_illegal_17, "illegal-17");
    test_parser_illegal!(test_illegal_18, "illegal-18");
    test_parser_illegal!(test_illegal_19, "illegal-19");
    test_parser_illegal!(test_illegal_20, "illegal-20");

}




