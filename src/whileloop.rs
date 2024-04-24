pub fn get_squares(limit: i32) {
    let mut num: i32 = 1;
    while num * num < limit {
        println!("{0} * {0} = {1}", num, { num * num });
        num += 1;
    }
}

pub fn get_cubes(limit: i32) {
    let mut num: i32 = 1;
    loop {
        println!("{0} * {0} * {0} = {1}", num, num * num * num);
        num += 1;
        if { num * num * num } > limit {
            break;
        }
    }
}
