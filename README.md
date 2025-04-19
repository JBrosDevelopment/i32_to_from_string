# Manual i32 Converter

This is a simple Rust program that manually converts between strings and 32-bit integers (`i32`) without using the standard `parse()` or `to_string()` methods. It's designed as a learning exercise in ASCII manipulation, error handling, and manual base-10 conversion.

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