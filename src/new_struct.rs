#[derive(Debug)]
struct Mobile{
    brand:u8,
    types:u8,
    modem:u8
}

impl Mobile{
    fn detail(&self)-> String{
        format!("brand :{}, type : {}, modem : {} ",self.brand,self.types,self.modem)
    }
}
impl Mobile{
    fn relat(brd:u8, typ:u8, mode:u8)-> Mobile{
        Mobile{
            brand:brd,
        types:typ,
        modem:mode
        }
    }
}
pub fn testerr(){
    let mi = Mobile{
        brand:55,
        types:85,
        modem:5
    };
    let ret_mob = Mobile::relat(22,56 ,88);
    println!("{:?}",  ret_mob);
    println!("{:?}",  mi.detail());
}