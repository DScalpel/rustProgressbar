use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {

    let mut total_size: i32 = 0;

    match term_size::dimensions() {
        Some((h, _)) => {
            //println!("Width: {}", &total_size);
            total_size = h as i32;
        }
        None => println!("Unable to get term size :("),
    }
    //total_size = total_size -5;
    println!("Total Size : {}", total_size);
    println!("Showing Progress...\n");
    for percent in 1..total_size + 1 {
        let ten_millis = time::Duration::from_millis(20);
        print!("[ ");
        for _ in 1..((percent * 100) / total_size) as i32 {
            print!("█");
        }
        (1..101 - ((percent * 100) / total_size)).for_each(|_| {
            print!("▓");
        });
        print!(" ]");
        if ((percent * 100) / total_size) == 100{
                print!(" [✓]");
        }
        else{
                print!(" {}%", (percent * 100) / total_size);
            
        }

        io::stdout().flush().unwrap();
        thread::sleep(ten_millis);
        if percent == total_size {
            println!("\n");
        } else {
            print!("\r");
        }
    }
}
