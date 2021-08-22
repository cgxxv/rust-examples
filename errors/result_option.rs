use std::num::ParseIntError;

// fn double_first(vec: Vec<&str>) -> i32 {
//     let first = vec.first().unwrap(); // 生成错误 1
//     2 * first.parse::<i32>().unwrap() // 生成错误 2
// }

/*
output:
The first doubled is Some(Ok(84))
The first doubled is None
The first doubled is Some(Err(ParseIntError { kind: InvalidDigit }))
*/
// fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
//     vec.first().map(|first| {
//         first.parse::<i32>().map(|n| 2 * n)
//     })
// }

/**
output:
The first doubled is Ok(Some(84))
The first doubled is Ok(None)
The first doubled is Err(ParseIntError { kind: InvalidDigit })
 */
fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2*n)
    }).map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));
}
