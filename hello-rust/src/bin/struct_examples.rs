#[derive(Debug)]
struct Rect {
    l1: f32,
    l2: f32,
}

impl Rect {

    fn area(&self) -> f32 {
        self.l1 * self.l2
    }
}

fn main() {

    let origin = Rect { l1: 1.9, l2: 2.0 };

    print!("{:?} are is {:?}\n",origin, origin.area())
}