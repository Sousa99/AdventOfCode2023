#[warn(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

#[allow(dead_code)]
pub fn read_lines(filename: String) -> Vec<String> {

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    return data;
}

#[allow(dead_code)]
pub fn read_int_lines(filename: String) -> Vec<i64> {

    let data_string: Vec<String> = read_lines(filename);
    let data: Vec<i64> = data_string.iter()
        .map(|line| line.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect::<Result<_, _>>().unwrap();

    return data;
}

#[allow(dead_code)]
pub fn read_list_int_lines(filename: String, separator: &str) -> Vec<Vec<i64>> {

    let data_string: Vec<String> = read_lines(filename);
    let data: Vec<Vec<i64>> = data_string.iter()
        .map(|line| line.split(separator)
            .map(|line| line.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
            .collect::<Result<_, _>>().unwrap())
        .collect::<Vec<Vec<i64>>>();

    return data;
}

#[allow(dead_code)]
pub fn read_digits(filename: String) -> Vec<Vec<u32>> {

    let data_string: Vec<String> = read_lines(filename);
    let data: Vec<Vec<u32>> = data_string.iter()
        .map(|line| line.chars()
            .map(|characther| characther.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    return data;
}

#[allow(dead_code)]
pub fn read_chars(filename: String) -> Vec<Vec<char>> {

    let data_string: Vec<String> = read_lines(filename);
    let data: Vec<Vec<char>> = data_string.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    return data;
}