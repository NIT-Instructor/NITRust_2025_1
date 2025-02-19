//Impliment battery structure

struct Battery {
    charge_level:u8,
    is_disposed:bool,
}

impl Battery {

    // fn new(charge_level:u8,is_disposed:bool) ->Battery
    // {
    //     Battery {charge_level,is_disposed}
    // }

    fn use_up(mut self: Battery)
    {
        println!("Battery use_up ed!!!");
        self.charge_level = 0;
        self.is_disposed = true
    }

    fn dispose(mut self: Battery)
    {
        if self.is_disposed == false
        {
            self.is_disposed = true;
            println!("Battery is Disposed!!!");
        }
        else{
            println!("Battery is Already Disposed!!!");
        }
    }
}

fn main()
{
    let battery = Battery{charge_level:40,is_disposed:false};

    battery.use_up();

   // battery.dispose();  //battery No longer usable,as ownership is tranfered

   let battery2 = Battery{charge_level:40,is_disposed:true};
   battery2.dispose();
  // battery2.dispose();// battery2 is un usable
  let battery3 = Battery{charge_level:40,is_disposed:false};
  battery3.dispose();
}