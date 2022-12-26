#[derive(Debug, PartialEq, PartialOrd)]
struct FooBar {
    x: f32,
    y: f32,   
}

#[allow(dead_code)]
impl FooBar {
    fn new(x: f32, y: f32) -> FooBar {
        FooBar {
            x: x,
            y: y,
        }
    }
}

fn sort_struct_vect() {
    let mut v: Vec<FooBar> = vec![
        FooBar::new(1.2, 0.0),
        FooBar::new(1.5, 0.0),
        FooBar::new(2.5, 0.0),
        FooBar::new(3.5, 0.0),
        FooBar::new(4.5, 0.0)
    ];
    
    println!("Before sorting = {:?}", v);
    v.sort_by(|a, b| b.x.partial_cmp(&a.x).unwrap());
    println!("After sorting = {:?}", v);
}


fn main() {
    sort_struct_vect();
}
