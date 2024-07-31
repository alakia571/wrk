/* patterns
fn func1_pattern(user_level: u8){
    match user_level {
        0 => println!("WELCOKE  0"),
        1 => println!("WELCOKE  1"),
        2 => println!("WELCOKE  2"),
        _ => println!("Error")
    }
}


#[derive(Clone)]
enum HttpStatus{
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500
}


fn print_status_pattern(status: HttpStatus){

    match status {
        HttpStatus::Ok => println!(" kheili ham awli !"),
        HttpStatus::NotFound => println!(" nabode keh !"),
        HttpStatus::InternalServerError => println!(" bad shod keh ")
    }
}


struct RgbStruct{
    red: u16,
    green: u16,
    blue: u16
}


fn color_exract_and_match(color: RgbStruct){
    match color{
        RgbStruct {red:r @ 0..=100, green:0, blue:0} => println!("This color is soft red : {}", r),
        RgbStruct {red:1, green:10, blue:10} => println!("1, 10 , 10"),
        _ => println!("Bad Req")
    }
}



fn get_pair(slice: &[i32]) -> (i32, i32) {
    match slice {
        [e1] => (*e1, *e1),
        [e1, e2, ..,e3, e4] => {
            let avg1 = (e1+e2).pow(2);
            let avg2 = (e1+e2).pow(2);
            (avg1, avg2)
        },
        [e1, .., e2] => (*e1, *e2),
        [] => (0,0)
    }
}
 */

/* Trait , interface, trait inheritance
trait Animal {
    fn eat(&self) {
        println!("all animals need to eat foods to alive!")
    }
}

trait Fish: Animal {
    fn moves(&self) {
        println!("fish should swim to move")
    }
}

struct Dolfine {
    name: String,
}

impl Fish for Dolfine {}

impl Animal for Dolfine {
    // fn eat(&self){
    //     println!("all animals need to eat foods to alive! 1111")
    // }
}
 */

/* struct with same method in multiple trait

trait Human{
    fn call_name(&self){
        println!("I am a human !")
    }

    fn walk(&self){
        println!("i can walk !")
    }
}


trait Man{
    fn call_name(&self){
        println!("I am a man")
    }

    fn shape(&self){
        println!("i a man with male body features!")
    }
}


struct Student{
    name: String,
    id: u16,
    age: u8
}



impl Human for Student{}

impl Man for Student{}

*/



fn main() {
    /* initial project    -------------------------------------------------------
    println!("سلام دنیا!");
    chars_display();
    array_display();
    array_print();
    tuple_display();
    simple_looper();
    simple_while();
    simple_for();
    array_itrate();
    print_star();
    loop_instead_while();
    adad_aval(1085);
    factorial(5);
    --------------------------------------------
    */

    /* patterns -----------------------------------------------------------------

    let user_inputs : [u8; 5] = [0, 1, 2, 3, 5];

    for user_input in user_inputs.iter() {
        func1_pattern(*user_input);
    };


    let status_list : [HttpStatus; 3] = [HttpStatus::Ok, HttpStatus::NotFound, HttpStatus::InternalServerError];

    for status in status_list.iter(){
        print_status_pattern(status.clone())
    };

    let sample_struct = RgbStruct{
        red: 100,
        green: 200,
        blue: 300
    };

    color_exract_and_match(sample_struct);

    let s_list = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut slice = &s_list[..];
    let mut pair = get_pair(slice);
    println!("created pair from {:?} is: {:?}", slice, pair);

    slice = &s_list(..=1);
    pair = get_pair(slice);
    println!("created pair from {:?} is: {:?}", slice, pair);


    slice = &a[..=4];
    pair = get_pair(slice);
    println!("Created pair from {:?} is: {:?}", slice, pair);

    slice = &a[0..1];
    pair = get_pair(slice);
    println!("Created pair from {:?} is: {:?}", slice, pair);

    slice = &[];
    pair = get_pair(slice);
    println!("Created pair from {:?} is: {:?}", slice, pair);

    slice = &a[..3];
    pair = get_pair(slice);
    println!("Created pair from {:?} is: {:?}", slice, pair);
    --------------------------------------------
    */

    /* Trait , interface, trait inheritance   -----------------------------------

    let eddi_dolfine = Dolfine {
        name: String::from("eddi"),
    };

    eddi_dolfine.moves();
    eddi_dolfine.eat();
    */

    /* struct with same method in multiple trait --------------------------------

    let ali = Student{
        name: String::from("ali"),
        id: 1,
        age: 12
    };

    Human::call_name(&ali);

    Man::call_name(&ali);
    */
}

/*
fn chars_display() {
    let a = 'e';
    let b = '1';
    let c = '‌'; //  نیم‌فاصله
    let d = 'پ';
    let e = '👀';
    println!("{} {} {}{}{} {} ", a, b, d, c, d, e);
}

fn array_display() {
    let _nums: [i8; 3] = [1, 2, 3];

    let months = [
        "فروردین",
        "اردیبهشت",
        "خرداد",
        "تیر",
        "مرداد",
        "شهریور",
        "مهر",
        "آبان",
        "آذر",
        "دی",
        "بهمن",
        "اسفند",
    ];

    let first_month = months[0];

    let last_month = months[11];

    println!("{}  {}", first_month, last_month)
}

fn array_print() {
    let arr_x = [10i8; 35];

    println!("my arrx is : {:?}", arr_x);
}

fn tuple_display() {
    let tup0: (i32, char, bool, f64);

    let _tup1 = (1, true, 'c', "hello");

    tup0 = (32, 'f', false, 2.65);

    println!("{}", tup0.0);
}

fn simple_looper() {
    let mut counter = 0;

    let a = loop {
        if counter == 5 {
            break counter;
        }
        counter += 1;
    };

    println!("a = {}", a)
}

fn simple_while() {
    let mut a = 1;

    while a % 10 != 0 {
        println!("a= {}", a);

        a += 1;
    }
}

fn simple_for() {
    for item in 2..11 {
        println!("item = {}", item);
    }
}

fn array_itrate() {
    let arr_y: [i32; 6] = [1, 2, 3, 4, 5, 6];

    for element in arr_y.iter() {
        println!("current item is : {}", element);
    }
}

fn print_star() {
    let head: i8 = 6;
    let mut star = String::new();

    for i in 0..head {
        println!("{}", star);
        star.push('*');
    }

    star.pop();

    while star.len() > 0 {
        star.pop();
        println!("{}", star);
    }
}

fn print_star_pyramid() {
    let head: i8 = 5;
    let mut star = String::new();

    // increase the number of asterisks
    for i in 0..head {
        star.clear(); // clear the string for each iteration
        for _ in 0..(i + 1) {
            star.push('*');
        }
        println!("{}", star);
    }

    // decrease the number of asterisks
    for i in (0..head - 1).rev() {
        star.clear(); // clear the string for each iteration
        for _ in 0..(i + 1) {
            star.push('*');
        }
        println!("{}", star);
    }
}

fn loop_instead_while() {
    let mut counter = 0;

    loop {
        if counter > 10 {
            break;
        }

        println!("counter : {}", counter);

        counter += 1;
    }
}

fn adad_aval(num: i32) -> i32 {
    let scope: i32 = (num as f64).sqrt() as i32;

    for i in 2..scope {
        println!("{}", i);
        if num % i == 0 {
            println!("aval nist in : {}", i);

            return i;
        }
    }

    return 0;
}

fn factorial(num: i32) -> i32 {
    if num == 1 || num <= 0 {
        return 1;
    }

    return num * factorial(num - 1);
}
    */
