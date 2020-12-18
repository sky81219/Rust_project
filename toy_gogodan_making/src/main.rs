use std::cmp::Ordering;

fn main() {
    match for_gogo_function().cmp(&while_gogo_function()) {
        Ordering::Less => println!("값이 작습니다!"),
        Ordering::Greater => println!("값이 큽니다!"),
        Ordering::Equal => println!("값이 같습니다!!")
    }

}

fn for_gogo_function() {
    println!("Welcome to for gogodan");
    for first_number in (1..10).rev(){
        for second_number in (1..10).rev(){
            let result = first_number + second_number;
            println!("{} * {} = {}", first_number, second_number, result)
        }
        println!("\n");
    }
}

fn while_gogo_function() {
    println!("Welcome to while gogodan");
    let mut _gogodan = 1;

    while 10 > _gogodan {
        let mut _count = 1;
        while 10 > _count {
            println!("{} * {} = {}", _gogodan, _count, _gogodan * _count);
            _count += 1;
        }
        _gogodan += 1;
        println!("\n");
    }
}

