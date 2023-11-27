pub mod model;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    for _j in 0..10 {
        for _i in 0..10000000 {
            str_fun();
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

//fn clear_str() {
//    let txt = String::from("some text");
//    model::write_model(&txt);
//}

fn str_fun() {
    let txt = return_str();
    model::write_model(&txt);
}

fn return_str() -> String {
    String::from("some text")
}
