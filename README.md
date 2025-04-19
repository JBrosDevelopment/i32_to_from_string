# Manual i32 Converter

This is a simple Rust program that manually converts between strings and 32-bit integers (`i32`) without using the standard `parse()` or `to_string()` methods. It's a little project in ASCII manipulation and manual base-10 conversion.

## Features

- Reads user input from the console
- Converts string input into an `i32` manually
- Converts an `i32` back into a string manually
- Detects and handles invalid input (e.g. non-digit characters, multiple `-` signs, overflows)
- Loops until the user enters a blank line

## Example
```
Input a valid i32 number (empty to quit):
12345
Result: 12345

Input a valid i32 number (empty to quit):
-6789
Result: -6789

Input a valid i32 number (empty to quit):
abc
Error, couldn't convert `abc` to i32
```

# Functions

```rs
fn i32_to_string(value: i32) -> Result<String, ()> {
    if value == 0 {
        return Ok("0".to_string());
    }

    let signed: bool = value < 0;
    let mut digits: Vec<u8> = Vec::new();
    let mut number: i32 = value.abs();

    while number >= 1 {
        digits.push((number % 10) as u8);
        number = number / 10;
    }
    
    digits.reverse();
    for i in 0..digits.len() {
        digits[i] = digits[i] + 48;
    }

    match String::from_utf8(digits) {
        Ok(v) => {
            let sign = if signed { "-".to_string() } else { "".to_string() };
            Ok(sign + &v)
        }
        Err(_) => {
            return Err(()); // failed to convert digits to string
        }
    }
}
```

```rs
fn string_to_i32(value: &String) -> Result<i32, ()> {
    let mut chars: Vec<char> = value.chars().collect::<Vec<char>>();
    chars.reverse();
    let mut number: i32 = 0;
    let mut negative: bool = false;
    let mut digits_stopped = false;
    
    for i in 0..chars.len() {
        if chars[i] == '-' {
            negative = !negative;
            digits_stopped = true;
            continue;
        }

        if digits_stopped {
            return Err(()); // digits out of order with negative sign
        }
        
        let character: i32 = chars[i] as i32;
        if character < 48 || character > 57 {
            return Err(()); // character is not valid digit '0'..'9'
        }

        let digit: i32 = character - 48;

        let power_of_ten = i32::pow(10, i as u32);
        if power_of_ten > i32::MAX / digit.clamp(1, 9) {
            return Err(()); // multiplication would overflow
        }

        let product = digit * power_of_ten;
        if product > i32::MAX - number {
            return Err(()); // addition would overflow
        }

        number += product;
    }

    if negative {
        Ok(-number)
    } else {
        Ok(number)
    }
}
```