fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];

    let lyrics = [
        "partridge in a pear tree",
        "2 turtle doves",
        "3 french hens",
        "4 calling birds",
        "5 golden rings",
        "6 geese a laying",
        "7 swans a swimming",
        "8 maids a milking",
        "9 ladies dancing",
        "10 lords a leaping",
        "11 pipers piping",
        "12 drummers drumming",
    ];

    for i in 0..days.len() {
        println!("On the {} day of Christmas", days[i]);
        println!("my true love sent to me:");

        for j in (0..=i).rev() {
            if j == 0 && i > 0 {
                print!("and a ");
            } else if i == 0 {
                print!("a ");
            }

            println!("{}", lyrics[j]);
        }

        if i + 1 < days.len() {
            print!("\n");
        }
    }
}
