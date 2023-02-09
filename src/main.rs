/*
MIT License
Copyright (c) 2023 m!haly4
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::io;

fn main() {

    println!("Choose an option:\n
             1] Convert C to F\n
             2] Convert F to C");

    let mut choise = String::new();

    io::stdin()
        .read_line(&mut choise)
        .expect("Failed to read your choise");

    let choise: u8 = choise.trim().parse().expect("Please type your choise");

    //branch for C
    if choise == 1 {
        println!("Please insert your temperature in C:");

        //variable for input
        let mut input_var = String::new();
    
        io::stdin()
            .read_line(&mut input_var)
            .expect("Failed to read the line");

        let input_var: f64 = input_var.trim().parse().expect("Please type your temperature");

        println!("{input_var}");
    }
    //branch for F
    else if choise == 2 {
        println!("Please insert your temperature in F:");

        //variable for input
        let mut input_var = String::new();

        io::stdin()
            .read_line(&mut input_var)
            .expect("Failed to read the line");

        let input_var: f64 = input_var.trim().parse().expect("Please type your temperature");

        println!("{input_var}");
    }
}
