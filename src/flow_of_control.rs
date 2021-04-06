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

    let triple = (0, -2, 3);
    println!("Tell me about {:#?}", triple);
    match triple {
        (0, y, z) => println!("First is '0','y' is {:?}, and 'z' is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
    }

    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    let pair = (2, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y)if x == y => println!("There are twins"),
        (x, y)if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 0 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"),
    }

    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    let a = Fou::Bar;
    let b = Fou::Baz;
    let c = Fou::Qux(100);

    if let Fou::Bar = a {
        println!("a is foobar");
    }

    if let Fou::Bar = b {
        println!("b is foobar");
    }


    if let Fou::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Fou::Qux(_value @ 100) = c {
        println!("c is one hundred");
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

enum Fou {
    Bar,
    Baz,
    Qux(u32),
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn age() -> u32 {
    15
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}