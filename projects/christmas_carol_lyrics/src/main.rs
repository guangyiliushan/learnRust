fn main() {
    println!("The Twelve days of Christmas.");
    let changes = [
        ("first", " partridge in a pear tree."),
        ("second", "Two turtle doves,"),
        ("third", " Three French hens,"),
        ("fourth", " Four calling birds,"),
        ("fifth", " Five gold rings,"),
        ("sixth", " Six geese a-laying,"),
        ("seventh", " Seven swans a-swimming,"),
        ("eighth", " Eight maids a-milking,"),
        ("ninth", " Nine ladies dancing,"),
        ("tenth", " Ten lords a-leaping,"),
        ("eleventh", " Eleven pipers piping,"),
        ("twelfth", " Twelve drummers drumming,"),
    ];
    let mut gifts = String::new();
    for (number, gift) in changes {
        if number == "first" {
            println!(
                "On the {} day of Christmas, my true love sent to me: A{}",
                number, gift
            );
            gifts.push_str(" And a");
            gifts.push_str(gift);
            continue;
        }
        gifts.insert_str(0,gift);
        println!(
            "On the {} day of Christmas, my true love sent to me:{}",
            number, gifts
        );
    }
}
