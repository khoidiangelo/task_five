use std::io;
fn main() {
    loop {
        println!("Enter numbers seperated by comma."); // Ask user for input
        let mut input_user = String::new();

        io::stdin()
            .read_line(&mut input_user)
            .expect("Failed to read line");

        let input_array: Vec<&str> = input_user.split(',').collect(); // split input by comma and save in an array

        let mut biggest_number: u32 = match input_array[0].trim().parse() {
            // convert first value to int
            Ok(num) => num,
            Err(_) => continue,
        };

        for index in 1..input_array.len() {
            //array loop to check for highest number

            let array_index_value: u32 = match input_array[index].trim().parse() {
                // convert value of "x" index to int for comparison
                Ok(num) => num,
                Err(_) => continue,
            };

            if array_index_value > biggest_number {
                // compare and if value bigger replace
                biggest_number = array_index_value;
            }
        }
        print!("\x1B[2J\x1B[1;1H"); // clears console
        println!("Highest number is: {biggest_number}"); // print highest number
        break;
    }
}