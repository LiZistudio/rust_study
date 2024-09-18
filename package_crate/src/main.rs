use package_crate::add;
use rand::*;
fn main() {
    let result1 = add(1,2);
    println!("{:?}",result1);

    let rng = &mut thread_rng();
    let boolean = rng.gen_bool(3.14);
    println!("{}",boolean);
}


//The first part of module system -- package and crate
