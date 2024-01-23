fn main() {
    let days_of_christmas: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts: [&str; 12] = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    println!("The Twelve Days of Christmas");

    for day in 0..12 {
        println!("On the {} day of Christmas,", days_of_christmas[day]);
        println!("My true love gave to me:");

        for gift_day in (0..=day).rev() {
            if gift_day == 0 && day > 0 {
                print!("And ");
            }
            println!("{}", gifts[gift_day]);

            if gift_day == 0 {
                println!(); // Empty line after each verse
            }
        }
    }
}
