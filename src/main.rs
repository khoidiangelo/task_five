use std::io;
fn main() {
    loop {
        println!("Enter numbers seperated by comma."); // Ask user for input
        let mut input_user = String::new();

        io::stdin()
            .read_line(&mut input_user)
            .expect("Failed to read line");

        let v: Vec<&str> = input_user.split(',').collect(); // split input by comma and save in an array

        let array_value = v[0]; // first value of array for comparison

        let mut comparison: u32 = match array_value.trim().parse() {    // convert first value to int
            Ok(num) => num,
            Err(_) => continue,
        };

        for index in 1..v.len() {   //array loop to check for highest number

            let array_index_value: u32 = match v[index].trim().parse() {    // convert value of "x" index to int for comparison
                Ok(num) => num,
                Err(_) => continue,
            };

            if array_index_value > comparison {     // compare and if value bigger replace
                comparison = array_index_value;
            }
            //println!("index: {}, value: {}", index, v[index]);  // debug shit ehe
        }
        print!("\x1B[2J\x1B[1;1H"); // clears console
        println!("Highest number is: {}", comparison); // print highest number
        break;
    }
}