pub fn flow_of_control() {
    let n = 5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else { println!("{} is zero", n); }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number,increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);


    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            // break;
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    let mut m = 1;
    while m < 101 {
        if m % 15 == 0 {
            println!("fizzbuzz");
        } else if m % 3 == 0 {
            println!("fizz");
        } else if m % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", m);
        }
        m += 1;
    }

    for i in 1..101 {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }

    let names = ["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name),
        }

        println!("names:{:#?}", names);
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name),
        }

        // println!("names:{:#?}", names);
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us",
            _ => "Hello",
        };
    }

    println!("names:{:#?}", names);

    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}