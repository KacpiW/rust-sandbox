// fn main() {
//     let mut x: i16 = 200;
//     while x != 1 {
//         if x % 2 == 0 {
//             x = x / 2;
//         } else {
//             x = 3 * x + 1;
//         }
//         print!(" -> {x}");
//     }
//     println!();
// }

// fn main() {
//     println!(r#"<a href="link.html">link</a>"#);
//     println!("<a href=\"link.html\">link</a>");
//     // println!("<a href="link.html">link</a>");
// }

// fn main() {
//     println!("{:?}", b"abc");
//     println!("{:?}", &[97, 98, 99]);
// }

// fn main() {
//     let mut a: [i8; 10] = [42; 10];
//     a[5] = 0;
//     println!("a: {:?}", a);
// }

// fn main() {
//     let t: (i8, bool) = (7, true);
//     println!("1st index: {}", t.0);
//     println!("2nd index: {}", t.1);
//     println!("Tuple: {:?}", t);
//     println!("Tuple: {:#?}", t);
// }

// fn main() {
//     let mut x: i8 = 10;
//     let ref_x: &mut i8 = &mut x;
//     *ref_x = 20;
//     println!("x: {}", x);
// }

// fn main() {
//     let ref_x: &i32;
//     {
//         let x: i32 = 10;
//         ref_x = &x;
//     }
//     println!("ref_x: {ref_x}");
// }

// fn main() {
//     let a: [i32; 6] = [1,2,3,4,5,6];
//     println!("a: {a:?}");
    
//     let s: &[i32] = &a[2..4];
//     println!("s: {s:?}");
// }

// fn main() {
//     let s1: &str = "World";
//     println!("s1: {s1}");

//     let mut s2: String = String::from("Hello ");
//     println!("s2: {s2}");
//     s2.push_str(s1);
//     println!("s2: {s2}");

//     let s3: &str = &s2[6..];
//     println!("s3: {s3}");
// }

/// Define FizzBuzz script
// fn main() {
//     print_fizzbuzz_to(20);
// }

// fn is_divisable(n: u32, divisor: u32) -> bool {
//     if divisor == 0 {
//         return false;
//     } 
//     n % divisor == 0
// }

// fn fizzbuzz(n: u32) -> String {
//     let fizz: &str = if is_divisable(n, 3) { "fizz" } else { "" };
//     let buzz: &str = if is_divisable(n, 5) { "buzz" } else { "" };
//     if fizz.is_empty() && buzz.is_empty() {
//         return format!("{n}");
//     }
//     format!("{fizz}{buzz}")
// }

// fn print_fizzbuzz_to(n: u32) -> () {
//     for i in 1..=n {
//         println!("{}", fizzbuzz(i));
//     }
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn inc_width(&mut self, delta: u32) {
//         self.width += delta;
//     }
// }

// fn new(width: u32, height: u32) -> Rectangle {
//     Rectangle { width, height }
// }

// fn main() {
//     let mut rect = Rectangle { width: 10, height: 5 };
//     println!("old area: {}", rect.area());
//     rect.inc_width(5);
//     println!("new area: {}", rect.area());
//     let mut rect2 = new(10, 10);
//     println!("rec2.width: {}, rec2.height: {}", rect2.width, rect2.height);
// }

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

fn main() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}