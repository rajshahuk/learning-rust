fn main() {
    let twelve_days = [ ("first", "A Partridge in a Pear Tree"),
                        ("second", "Two Turtle Doves"),
                        ("third", "Three French Hens"),
                        ("fourth", "Four Calling Birds"),
                        ("fifth", "Five Golden Rings"),
                        ("sixth", "Six Geese a Laying"),
                        ("seventh", "Seven Swans a Swimming"),
                        ("eighth", "Eight Maids a Milking"),
                        ("ninth", "Nine Ladies Dancing"),
                        ("tenth", "Ten Lords a Leaping"),
                        ("eleventh", "Eleven Pipers Piping"),
                        ("twelfth", "12 Drummers Drumming")];

    let mut index = 0;
    while index < twelve_days.len() {
        let item = twelve_days[index];
        println!("On the {} day of Christmas my true love gave to me {}", item.0, item.1);
        let mut day: i8 = (index as i8) - 1;
        while day >= 0 {
            if day == 0 {
                print!("and a ")
            }
            println!("{}", twelve_days[day as usize].1);
            day -= 1;
        }
        index += 1;
        println!();
        
    }
}
