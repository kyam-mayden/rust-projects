fn main() {
    let days_of_christmas = [
        ["First", "A partridge in a pear tree"],
        ["Second", "Two turtle doves"],
        ["Third", "Three French hens"],
        ["Fourth", "Four calling birds"],
        ["Fifth", "FIIIIIIVE GOOOOLD RIIINGS"],
        ["Sixth", "Six geese a-laying"],
        ["Seventh", "Seven swans a-swimming"],
        ["Eigth", "Eight maids a-milking"],
        ["Ninth", "Nine ladies dancing"],
        ["Tenth", "Ten lords a-leaping"],
        ["Eleventh", "Eleven pipers piping"],
        ["Twelfth", "Twelve drummers drumming"],
    ];

    for (i, day) in days_of_christmas.iter().enumerate() {
        println!("On the {} day of Christmas, my true love sent to me:", day[0]);
        println!("{}", day[1]);
        let mut count = i;

        while count >1 {
            count = count - 1;
            println!("{}", days_of_christmas[count][1]);
        }
        
        if i != 0 {
            println!("And {}", days_of_christmas[0][1].to_lowercase());   
        }

        println!("");
    }
}
