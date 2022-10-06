fn main() {
    for day in 1..13 {
        print_day(day);
        for chorus in 0..day {
            print_chorus((day-chorus-1).try_into().unwrap());
        }
    }
}

fn print_day(day: u32) {
    let digit = day % 10;
    if day < 20 && day > 10 {
        println!("On the {day}th day of Christmas my true love gave to me:\n");
    } else {
        let string = match digit {
        1 => format!("{}st", day),
        2 => format!("{}nd", day),
        3 => format!("{}rd", day),
        _ => format!("{}th", day),
        };
        println!("On the {string} day of Christmas my true love gave to me:\n");
    }
}

fn print_chorus(day: usize) {
    let lyric = [
        "A partridge in a pear tree\n",
        "Two turtle doves, and\n",
        "Three french hens\n",
        "Four calling birds\n",
        "Five golden rings\n",
        "Six geese a-laying\n",
        "Seven swans a-swimming\n",
        "Eight maids a-milking\n",
        "Nine ladies dancing\n",
        "Ten lords a-leaping\n",
        "Eleven pipers piping\n",
        "Twelve drummers drumming\n"
    ];
    println!("{}",lyric[day]);
}
