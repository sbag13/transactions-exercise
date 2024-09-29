use std::collections::HashSet;

use test_case::test_case;

use crate::process_file;

#[test_case("file_without_spaces.csv", ["1,1.5,0,1.5,false", "2,2,0,2,false"]; "file without spaces")]
#[test_case("type_case_insensitivity.csv", ["1,1.5,0,1.5,false", "2,2,0,2,false"]; "type case insensitivity")]
#[test_case("file_with_spaces.csv", ["1,1.5,0,1.5,false", "2,2,0,2,false"]; "file with spaces")]
#[test_case("precision_up_to_4_decimal.csv", ["1,2000000000.1235,0,2000000000.1235,false"]; "precision up to 4 decimal")]
fn test_file_without_white_spaces<const N: usize>(file_name: &str, expected_lines: [&str; N]) {
    let mut buf = Vec::new();
    process_file(format!("./test_files/{file_name}"), &mut buf).unwrap();

    let result = String::from_utf8(buf).expect("Invalid UTF-8");
    let result_lines: HashSet<&str> = result.lines().skip(1).collect(); // Skip header

    let expected_lines = HashSet::from(expected_lines);
    assert_eq!(result_lines, expected_lines);
}
