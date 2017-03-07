trait SpicyMeatball {
    fn get_msg(&self) -> String;
}

impl SpicyMeatball for bool {
    fn get_msg(&self) -> String {
        if *self {
            String::from("mamma mia")
        } else {
            String::from("that's a spicy meatball")
        }
    }
}

impl SpicyMeatball for i32 {
    fn get_msg(&self) -> String {
        format!("mamma mia, that meatball is {} times too spicy", *self)
    }
}

fn main() {
    println!("{}, {}", true.get_msg(), false.get_msg());
    println!("{}", 25.get_msg());
}
