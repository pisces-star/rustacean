use std::fmt;
use std::fmt::{Formatter, Display};
use std::f32::consts::PI;

pub fn by_example(){
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    let x = 5 + 90 + 5;
    println!("Is `x` 10 or 100? x = {}", x);
    println!("{} days", 31);
    println!("{0}, this is {1}.{1},this is {0}", "Bob", "John");
    println!("{object} {subject} {verb}", object = "My name", subject = "is", verb = "pisces");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>0width$}", number = 1, width = 6);
    println!("My name is {0}, {1} {0}", "Bond", "John");
    println!("This struct `{:#?}` won't print...", Structure(3));
    println!("Pi is roughly {:.3}", PI);
    println!("{1} {0} is the {actor} name.",
             "Slater",
             "Christian",
             actor = "actor's");

    let name = "Peter";
    let age = 25;
    let peter = Person { name, age };

    println!("{:#?}", peter);

    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Display:{}", complex);
    println!("Debug:{:#?}", complex);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }
}

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{real} + {imag}i", real = self.real, imag = self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{0}:{1}", count, v)?;
        }
        write!(f, "]")
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({red}, {green}, {blue}) ox{red:02X}{green:02X}{blue:02X}",
               red = self.red, green = self.green, blue = self.blue)
    }
}
