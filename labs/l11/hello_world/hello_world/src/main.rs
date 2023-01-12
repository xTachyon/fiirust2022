use hello_world_lib::Dbg;

#[derive(Dbg)]
struct S {
    x: u32,
    y: String,
}

fn main() {
    let s = S {
        x: 5,
        y: "abc".to_string(),
    };
    println!("{:?}", s);
}
