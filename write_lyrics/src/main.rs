
fn main() {
    write_twelfth_day_of_christmas();
}

fn write_twelfth_day_of_christmas() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese-a-laying",
        "Seven swans-a-swimming",
        "Eight maids-a-milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve lords-a-leaping"
    ];
    let ordinal_numbers = [
        "first",    // 1-й
        "second",   // 2-й
        "third",    // 3-й
        "fourth",   // 4-й
        "fifth",    // 5-й
        "sixth",    // 6-й
        "seventh",  // 7-й
        "eighth",   // 8-й
        "ninth",    // 9-й
        "tenth",    // 10-й
        "eleventh", // 11-й
        "twelfth"   // 12-й
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas\nMy true love gave to me", ordinal_numbers[i]);
        for j in (0..=i).rev() {
            println!("{}",gifts[j]);
        }
        println!("\n");
    }

}
