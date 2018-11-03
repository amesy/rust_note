fn main() {
    let mut x = 5; 
    println!("the x value is {0}", x);

    x = 6; 
    println!("the x value is {0}", x);

    const AGE: u32 = 23; 
    println!("the amesy's age is {0}", AGE);

    let y = 6; 
    let y = y + 2; 
    let y = y * 3; 
    println!("the y's value is {0}", y);

    let name = "         "; 
    let name = name.len(); 
    println!("the name's value is {0}", name);


    let newyear = 5;
    info(newyear);   

    let number = {
        let num = 100; 
        num + 1 
    };
    println!("the num's value is: {0}", number);

    let five = five();
    println!("the five value is: {0}", five); 

    let multifive = multifive(5); 
    println!("the multifive value is {0}", multifive);

    // streamif();

    // streamloop(2);
    streamwhile();

    array();
    array_in()
}

fn info(newyear: i32) { 
    let name = "amesy"; 
    let age = 23; 
    let age_five = age + newyear; 
    println!("your info: {0}, {1}, {2}", name, age, age_five);
}

fn five() -> i32 { 
    5
}

fn multifive(x: i32) -> i32 { 
    x + 550
}

fn streamif() { 
    let con = true; 
    let conn_end = if con { 
        1000
    } else { 
        2000
    };

    println!("the streamif value is: {}", conn_end)
}

fn streamloop(x: i32) { 
    loop {
        println!("again: {}", x);
    }; 
}

fn streamwhile() { 
    let mut age = 25;
    while age <= 30 { 
        println!("you are so young and your age is: {0}", age); 
        age = age + 1
    };
}

fn array() { 
    let arrays = [30, 40, 50, 60, 70, 80, 90]; 
    let mut index = 1;

    while index < 6 { 
        println!("the array value is: {}", arrays[index]); 
        index = index + 1
    };
}

fn array_in() { 
    let arrays = [30, 40, 50, 60, 70, 80, 90]; 
    // if 50 in arrays.iter() { 
    //     println!("OK");
    // }
    println!(arrays.iter());
}
