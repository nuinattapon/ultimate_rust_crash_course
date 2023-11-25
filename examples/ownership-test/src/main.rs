fn main() {
    // let mut x = vec![1, 2, 3];

    // x.push(4);
    // let last: &i32 = x.last().unwrap();
    // println!("{:?}", last);
    // println!("{:?}", x);

    let mut s1 = String::from("abc");
    println!("{}", s1);

    do_stuff(&mut s1);
    println!("{}", s1);
}

fn do_stuff(s: &mut String) {
    s.insert_str(0, "Hi, ");
    *s = String::from("Replacement");
    // println!("{}", s);
}
