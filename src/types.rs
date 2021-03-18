pub fn types(){
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }

    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);

    println!("{:?}", vec);
}