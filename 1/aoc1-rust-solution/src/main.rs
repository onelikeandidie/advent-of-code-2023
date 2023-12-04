// The objective is to read a file line by line and extract the first and last
// digit on each line to make a number
// Something to do with a trebuchet idk
fn main() {
    // Get the arguments passed to the program
    let args = std::env::args();
    // Look for the path arg
    // Probably could have used iter.find() for this
    let mut path_is_next_arg = false;
    let mut path: Option<String> = None;
    for arg in args {
        if path_is_next_arg {
            path = Some(arg.to_string());
            break;
        }
        match arg.as_str() {
            "--path" | "-p" => path_is_next_arg = true,
            _ => {}
        }
    }
    // Ensure path is passed and read the file contents
    let path = path.expect("No --path to puzzle input found!");
    let file = std::fs::read_to_string(path).unwrap();
    // Method chain like crazy!
    let total: i32 = file
        .lines()
        // Break each line into chars
        .map(|line| line.chars())
        // Filter only numeric chars
        .map(|line_chars| line_chars.filter(|c| c.is_numeric()))
        // Map each line to a list of numbers
        .map(|line_numbers| line_numbers.collect::<Vec<char>>())
        .map(|line_numbers| {
            // Pop the first and last numbers
            let first = line_numbers.first();
            let last = line_numbers.last();
            // Create a number string
            let mut number = String::new();
            // Get each number and add it to the string
            if let Some(first) = first {
                number.push(*first);
            }
            if let Some(last) = last {
                number.push(*last);
            }
            // Parse numbers into i32
            number.parse::<i32>().unwrap_or(0)
        })
        // Calculate total
        .sum();
    // Print out the result
    println!("{}", total);
}
