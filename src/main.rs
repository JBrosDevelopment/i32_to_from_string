fn main() {
    println!();
    // get input from user
    println!("Input a valid i32 number (empty to quit):");
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        input = input.trim().to_string();
    } else {
        println!("Error, could not read input");
        main();
    }

    if input.is_empty() {
        // end program
        return;
    }
    
    // convert inputted string to i32
    let converted_i32: i32 = match string_to_i32(&input) {
        Ok(result) => result,
        Err(_) => {
            println!("Error, couldn't convert `{}` to i32", input);
            main();
            return;
        }
    };

    // convert the newly created i32 back to string
    let converted_string = match i32_to_string(converted_i32) {
        Ok(result) => result,
        Err(_) => {
            println!("Error, couldn't convert `{}` to string", converted_i32);
            main();
            return;
        }
    };

    // print the result
    println!("Result: {}", converted_string);
    main();
}

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