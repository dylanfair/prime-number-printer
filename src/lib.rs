/// This will only work for numbers under 10 million or so
pub fn sieve_of_eratosthenes(number: usize) -> Vec<usize> {
   let mut timevec = vec![true;number];
   let mut primevec = vec![];

   let mut p = 2;
   while p * p <= number {
      if timevec[p] == true {
         let mut i = p*p;
         let inner_range = p*p..number;
         while inner_range.contains(&i) {
            timevec[i] = false;
            i += p;
         }
      }
      p += 1;
   }
   for p in 2..number {
      if timevec[p] {
         primevec.push(p);
      }
   }

   return primevec
}

pub fn from_user_get_positive_number() -> usize {
   loop {
      let mut response = String::new();
      std::io::stdin().read_line(&mut response).expect("Failed to read response.");
      let number = response.replace("\n", "");
      match number.parse::<usize>() {
         Ok(n) => {
            if n >= 10_000_000 {
               println!("Please choose a smaller integer, prime calculations will be off");
            }
            else {
               return n
            }
         }
         Err(err) => {
            println!("{}", err);
            println!("Please input a positive integer");
         }
      }
   }

}

