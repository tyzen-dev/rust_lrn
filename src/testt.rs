struct Anniyan {
    ambi: String,
    remo: String,
    anniyan: String,
}

impl Anniyan {
    fn details(&self) -> String {
        format!("hai {} how are you.!!!", self.ambi)
    }
}
impl Anniyan {
    fn privt(a1: &str, r1: &str, a3: &str) -> Anniyan {
        Anniyan {
            ambi: a1.to_string(),
            remo: r1.to_string(),
            anniyan: a3.to_string(),
        }
    }
}
pub fn messager() {
    let persion = Anniyan {
        ambi: String::from("Ambi"),
        remo: String::from("remo"),
        anniyan: String::from("anniyan"),
    };
    println!("hello {:?} ", persion.details());
    let temp = Anniyan::privt("AMBI", "REMO", "ANNIYAN");
}
