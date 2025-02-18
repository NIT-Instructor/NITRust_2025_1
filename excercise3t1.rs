
//Task 1: Implement Vehicle Trait
trait vehicle_trait{
    fn start_engine(&self)->String;
    fn stop_engine(&self)->String;
}
struct car {
    name:String,
}

struct truck {
    name:String,
}

impl vehicle_trait for car  {

    fn start_engine(&self)-> String {
            let ret_str = String::from(self.name.clone());
            ret_str
        }

    fn stop_engine(&self) -> String {
        let ret_str = String::from(self.name.clone());
        ret_str

    }
}

impl vehicle_trait for truck  {

    fn start_engine(&self)->String {
            let ret_str = String::from(self.name.clone());
            ret_str
        }

    fn stop_engine(&self)->String {
        let ret_str = String::from(self.name.clone());
        ret_str
    }
}


fn main()
{
    println!("Excerise3-t1");
    let v1 =  car { name: String::from("car") };
    let str1 = v1.start_engine();
    println!("Starting in Main {}!", str1);
    let strstop = v1.stop_engine();
    println!("Stopping {}",strstop);

    let v2 = truck{ name:String::from("Truck") };
    let str2 = v2.start_engine();
    println!("Starting  {}!", str2);

    let strstop2 = v2.stop_engine();
    println!("Stopping {}",strstop2);

}