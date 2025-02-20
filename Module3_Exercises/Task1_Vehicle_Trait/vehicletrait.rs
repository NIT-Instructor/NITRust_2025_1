
trait Vehicle{
    fn start_engine(&self)->String;
    fn stop_engine(&self)->String;
}
struct Car {
    name:String,
    model:String,
}

struct Truck {
    name:String,
    model:String,
}

impl Vehicle for Car  {

    fn start_engine(&self)-> String {
            let ret_str = format!("Car {} {} Started !!! ",self.name,self.model);
            ret_str
        }

    fn stop_engine(&self) -> String {
        let ret_str = format!("Car {} {} Stopped !!! ",self.name,self.model);
        ret_str

    }
}

impl Vehicle for Truck  {

    fn start_engine(&self)->String {
            let ret_str = format!("Truck {} {} Started !!! ",self.name,self.model);
            ret_str
        }

    fn stop_engine(&self)->String {
        let ret_str = format!("Truck {} {} Stopped !!! ",self.name,self.model);
        ret_str
    }
}


fn main()
{
    let v1 =  Car { name: String::from("Maruti"),model: "Gypsy".to_string() };
    let str1 = v1.start_engine();
    println!("{}", str1);
    let strstop = v1.stop_engine();
    println!("{}",strstop);

    let v2 = Truck{ name:String::from("Tata"),model:"Signa".to_string()};
    let str2 = v2.start_engine();
    println!("{}", str2);

    let strstop2 = v2.stop_engine();
    println!("{}",strstop2);

}