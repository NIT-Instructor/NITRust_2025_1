//------------------TASK1 Find Min-Max-----------------------//

fn find_min_max(numbers: &[i32]) ->(i32,i32) {
    //return 0 if array is empty.
    if numbers.is_empty() {
        println!("EMPTY ELEMENTS");
        return (0,0);
    }

    let mut max_num = numbers[0];

    let mut min_num = numbers[0];

    // Iterate over the array to find the maximum number
    for &num in numbers {

        if num > max_num {
            max_num = num;
        } 

        if num < min_num {
            min_num = num;
        }   
        
    }
   // Some(max_num)
   return (max_num,min_num);
}
//------------------TASK1 Find Min-Max-----------------------//

//------------------Task 2: Vehicle Management System--------//


struct Vehicle {
    id:i32,
    make: String,
    year: i32,
    mileage:i32,
    vehicleType:VehicleType
}

enum VehicleType {
    car, 
    truck, 
    motorcycle
}

//store vehicles in vecotr Vec<Vehicle> & use references to manipulate the data.
//static mut vehicle_db: Vec<Vehicle> = Vec::new();

//static mut vehicle_db: Vec<Vehicle> = vec![];
static mut vehicle_db: Vec<Vehicle> = vec![];



//fn addNewVehicle(v1:Vehicle)
fn add_new_vehicle(vehicle:Vehicle)
{
    println!("Vehicle -id, {}! ,Make- {} ,Year- {},Mileage-{}",
        vehicle.id, vehicle.make,vehicle.year,vehicle.mileage);
        unsafe 
        { 
            vehicle_db.push(vehicle);
        }

}
fn remove_by_id(id:i32)
{
    unsafe {

    for item in &vehicle_db {
        let mut index =0;
            if item.id == id
            {
                unsafe { 

                vehicle_db.remove(index);//TODO tremove element
                }

                println!("item Removed {}", item.id); // Iterate elements

            }
        }

    }
}

fn list_All()
{
    println!("List Contents");
    unsafe {
    for item in &vehicle_db {
        println!("{}", item.id); // Iterate elements
        }

    }
}
fn query_by_make_year(make:String,year:i32)
{
    unsafe {
    for item in &vehicle_db {
         // Iterate elements
            if item.year == year
            {
                println!("Match Found {}",item.year);
            }
            //compare strings

            if make == item.make
            {
                println!("Match Found Make {}",item.make);

            }
            // match item.make {
            //     make => println!("Match Found for make{}",item.make),
            //     _ => println!("nothing"),
            // }
        }

    }
}



//------------------Task 2: Vehicle Management System--------//


//TASK3-CAN PASRSER  START

// enum CanMessage
// {
//     EngineStatus{
//         rpm:u8,
//         coolant_temp:u8,
//         oil_pressure:u8  
//     },
//     BrakeSystem{
//         brake_pressure:u8,
//         abs_active:bool   
//     },
//     Transmission{
//         gear_position:u8,
//         clutch_engaged:bool
//     },
//     Unknown
// };

// EngineStatus: Contains fields like rpm, coolant_temp, and oil_pressure.
// BrakeSystem: Contains fields like brake_pressure and abs_active.
// Transmission: Contains fields like gear_position and clutch_engaged.

// struct parsedmsg
// {
//     variant:CanMessage,
//     es:EngineStatus,
//     bs:BrakeSystem,
//     ts:Transmission
// }


// fn parse_can_message([u8:8])->parsedmsg
// {
//     for i in msg
//     {
//         println!("msg {}",i);
//         // if i == 0x01
//         // {
//         //     println!("Engine Status");
//         //     parsedmsg.type = i;

//         // }
//     }
//     return 

// }

// fn process_message(msg[u8:8])
// {
//     println!("DO NOTHING");

// }
//TASK3-CAN PASRSER  END
fn main()
{

//------------------TASK1 Find Min-Max-----------------------//
    let integers: [i32; 6] = [6, 8, 1, 3, 5, 9];

    let integers2:[i32;9] = [4,7,8,33,66,10,99,77,2];

    let integers3:[i32;0] = [];


    let mut minMax:(i32,i32) = (0,0);
    

    minMax = find_min_max(&integers);
    println!("MAX & MIN= {:?}", minMax);

    minMax = find_min_max(&integers2);
    println!("MAX & MIN= {:?}", minMax);

    minMax = find_min_max(&integers3);
    println!("MAX & MIN= {:?}", minMax);

 //------------------TASK1--end-----------------------//
//TASK2--STRAT--

    let mut v1 = Vehicle {
        id:1,
        make: String::from("honda"),
        year: 2024,
        mileage:15,
        vehicleType:VehicleType::car
    };

    let mut v2 = Vehicle {
        id:2,
        make: String::from("maruti"),
        year: 2025,
        mileage:20,
        vehicleType:VehicleType::car
    };

    add_new_vehicle(v1);
    add_new_vehicle(v2);


    let mk = String::from("honda");
    query_by_make_year(mk,2024);

    list_All();
    remove_by_id(2);
    list_All();

    //TASK2---END

    //TASK3-START

    // let messages = [

    // [0x01, 0x1F, 0x40, 90, 45, 0, 0, 0], // EngineStatus

    // [0x02, 80, 1, 0, 0, 0, 0, 0],       // BrakeSystem

    // [0x03, 3, 1, 0, 0, 0, 0, 0],        // Transmission

    // [0x04, 1, 1, 1, 1, 1, 1, 1]         // Unknown

    // ];

    // for msg in messages {

    // let parsed_msg = parse_can_message(msg);

    //     process_message(parsed_msg);

    // }

//TASK3-END


}