fn main() {
   println!("Give me a number:");
   let number = prime_number_printer::from_user_get_positive_number();

   println!("The primes under that number are:");
   let primes = prime_number_printer::sieve_of_eratosthenes(number);
   println!("{:?}", primes);

   println!("The following prime pairs equal your number:");
   let primepairs = prime_number_printer::find_prime_pairs(number, primes);
   println!("{:?}", primepairs);
}
