
#[warn(unused_variables, dead_code)]

// cannot find how to print own obj
#[derive(Debug)]
struct Persion {
    height: u8,
    wight: u8,
}
// Methods
impl Persion {
    // normal impl to adding two strings
    fn details(&self) -> String {
        format!("{}--{}", self.height, self.wight)
    }
}

// Realated fucntion
impl Persion {
    // Realated function, not use self, arg pass
    // as like normal fn
    // modified Persion struct, using agument
    fn mouse(h: u8, w: u8) -> Persion {
        Persion {
            height: h,
            wight: w,
        }
    }
}
pub fn structs() {
    let somu = Persion {
        height: 4,
        wight: 60,
    };

    //  is not normal fn call Persion.mouse(33,44)
    // is Realated fn
    let tony = Persion::mouse(33, 44);

    println!(
        "height : {}, wight : {} Fn : {:?}",
        somu.height,
        somu.wight,
        somu.details()
    );
    println!(
        "height : {}, wight : {} Fn : {:?}",
        tony.height,
        tony.wight,
        tony.details()
    );
    println!("{:#?}", somu);
}
