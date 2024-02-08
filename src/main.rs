fn is_prime(number: i128) -> bool {
    
    

    
    //check if the number is divisible by 2
    if number % 2 == 0 {
        return false;
    }

    //check if the number is divisible by any odd number from 3 to the square root of the number
    for i in (3..(number as f64).sqrt() as i128 + 1).step_by(2) {
        if number % i == 0 {
            return false;
        }
    }

    //return true if the number is prime
    return true;
}

fn main() {
  
    let mut max_number = String::new();
    println!("Enter the max number you want: ");
    std::io::stdin().read_line(&mut max_number).expect("Failed to read line");
    let max_number: i128 = max_number.trim().parse().expect("Please type a number!");

    //create a vector to store the prime numbers
    let mut prime_numbers: Vec<i128> = Vec::new();

    let start = std::time::Instant::now();

    //loop through all the numbers from 3 to max_number with a step of 2
    for i in (3..max_number).step_by(2) {
        //check if the number is prime
        if is_prime(i) {
            //add the number to the vector
            prime_numbers.push(i);
            println!("{}", i);
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    
}
