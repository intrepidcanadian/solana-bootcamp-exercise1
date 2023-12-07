fn main() {
    println!("Hello, world!");
    fizz_buzz();
}

fn fizz_buzz() {

    let mut fizz_buzz_count = 0;

    for i in 1..=301 {
        match (i%3, i%5) {
            (0,0) => {

                println!("fizzbuzz");
                fizz_buzz_count += 1;
            }
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => (),

        }

    }


    println!("'fizz buzz' occurred {} times", fizz_buzz_count);
}
