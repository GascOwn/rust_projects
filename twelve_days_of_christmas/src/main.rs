fn main() {

    let lines = ["A partridge in a pear tree", 
                 "Two turtle-doves", 
                 "Three French hens",
                 "Four calling birds",
                 "Five golden rings (five golden rings)",
                 "Six geese a laying",
                 "Seven swans a swimming",
                 "Eight maids a milking",
                 "Nine ladies dancing",
                 "Ten lords a-leaping",
                 "I sent 11 pipers piping",
                 "12 drummers drumming"
    ];

    let mut i = 0;
    let mut counter;

    while i < 12 {

        match i {
            0 => counter = "st",
            1 => counter = "nd",
            2 => counter = "rd",
            _ => counter = "th"           
        }
        
        println!("On the {}{} day of Christmas \nMy true love sent to me", (i + 1), counter);

        for number in (0..i + 1).rev(){
            println!("{}", lines[number]);
        }
        println!("\n");
        i = i + 1;
    }

}
