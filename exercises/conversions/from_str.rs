// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}


// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 如果输入字符串为空，就返回ParsePersonError::Empty错误
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }
        // 用逗号分隔字符串，得到一个包含两个元素的向量
        let parts: Vec<&str> = s.split(',').collect();
        // 如果向量的长度不等于2，就返回ParsePersonError::BadLen错误
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }
        // 取出向量的第一个元素作为名字
        let name = parts[0];
        // 如果名字为空，就返回ParsePersonError::NoName错误
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }
        // 取出向量的第二个元素作为年龄
        let age = parts[1];
        // 尝试把年龄转换成usize类型，如果失败，就返回ParsePersonError::ParseInt错误，并包装原始的错误信息
        let age = age.parse::<usize>().map_err(ParsePersonError::ParseInt)?;
        // 创建一个Person结构体，并返回Ok结果
        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}