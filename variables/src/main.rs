


const MAX_POINTS:u32 = 1000;

fn main() {
    let mut x = 4;
    println!("x = {}", x);
    x = 8;
    x = x + 2;
    println!("x = {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces {}", spaces);

    let f = 2.0;
    let d: f32 = 3.0;

    let tupl: (i32, f64, u8) = (500, 6.2, 1);
    let (x,y,z) = tupl;

    println!("its {} {} {}",x,y,z);
    println!("{}*{}*{}", tupl.0, tupl.1, tupl.2);
}
