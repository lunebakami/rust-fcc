// fn main() {
// Exercise 1 - Create a list and add 1 to each item
// let foo: Vec<_> = vec![1,2,3].
//             iter()
//             .map(|x| x + 1)
//             .collect();
// println!("{:?}", foo);
// .collect() explanation
// let items = vec![1, 2, 3];
// let mut iter = items
//                 .iter()
//                 .map(|x| x + 1);
//
// let mut collected_items = vec![];
// while let Some(value) = iter.next() {
//     collected_items.push(value)
// }
//
// println!("{:?}", collected_items);

// Exercise 2 - Read lines from file
// let file = std::fs::read_to_string("./lines").unwrap();
//
// file
//     .lines()
//     .enumerate()
//     .filter(|(idx, _)| idx % 2 == 0)
//     .skip(2)
//     .take(2)
//     .for_each(|(_, line)| println!("{}", line));
// }

// Exercise 3 - Enums
// enum Color {
//     Red,
//     Green,
//     Blue,
//     Yellow
// }
//
// impl Color {
//     fn is_green(&self) -> bool {
//         if let Color::Green = self {
//             return true;
//         }
//
//         return false;
//     }
//
//     fn is_green_parts(&self) -> bool {
//         match self {
//             Color::Red => false,
//             Color::Green => false,
//             Color::Blue => true,
//             Color::Yellow => true
//         }
//     }
// }
//
// fn print_color(color: Color) {
//     match color {
//         Color::Red => println!("red"),
//         Color::Green => println!("green"),
//         Color::Blue => println!("blue"),
//         Color::Yellow => println!("yellow")
//     };
// }

// Exercise 4 - Enums 2
// struct Custom {
//     age: usize,
//     name: String
// }
//
// enum Item {
//     Number(usize),
//     String(String),
//     MyCustom(Custom)
// }
//
// fn append(items: &mut Vec<Item>) {
//     items.push(Item::String("Hello Fem!".into()))
// }
//
// fn main() {
//     let mut items: Vec<Item> = vec![];
//
//     append(&mut items);
// }

// Exercise 5
// fn practice(nums: Vec<usize>, index: usize) -> usize {
//     return nums.get(index).unwrap_or(&index) * 5_usize;
// }
// fn main() {
//     let a = practice(vec![1,2,3,4,5], 4_usize);
//     println!("{}", a)
// }

// Exercise 6
// fn main() {
//     let file_name = std::env::args().nth(1)
//         .expect("the filename to be passed in");
//
//     let file = std::fs::read_to_string(file_name)
//         .expect("unable to read the file to string!");
//
//     file.lines().for_each(|line| {
//         if let Ok(value) = line.parse::<usize>() {
//             println!("{}", value);
//         } else {
//             println!("Line not a number")
//         }
//     });
// }

// Exercise 7
// #[derive(Debug)]
// struct Item {
//     count: usize
// }
//
// fn add_one(item: &mut Item) {
//     item.count += 1;
// }
//
// fn print_all(items: &Vec<Item>) {
//     for item in items {
//         println!("{:?}", item);
//     }
// }
//
// fn main() {
//     let mut items = vec![Item { count: 1}];
//     let first = items.get_mut(0);
//     let second = items.get_mut(1);
//     println!("{:?}", second);
// }

// Exercise 8
mod shapes;

use std::{str::FromStr, fmt::Display};

use anyhow::Result;
use shapes::collisions::{Points, Contains};

use crate::shapes::{circle::Circle, rect::Rect,collisions::Collidable};

enum Shape {
    Circle(Circle),
    Rect(Rect)
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

        match shape {
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            "rect" => return Ok(Shape::Rect(data.parse()?)),
            _ => return Err(anyhow::anyhow!("bad shape")),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Circle(c) => return c.points(),
            Shape::Rect(r) => return r.points(),
        }
    }
}

impl Contains for &Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => return c.contains_point(point),
            Shape::Rect(r) => return r.contains_point(point),
        }   
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle(c) => return write!(f, "{}", c),
            Shape::Rect(r) => return  write!(f, "{}", r),
        }
    }
}

fn main() -> Result<()> {

    let shapes = std::fs::read_to_string("../shapes")?
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    shapes
        .iter()
        .skip(1)
        .zip(shapes
            .iter()
            .take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(b))
        .for_each(|(a, b)| {
            println!("{} collides with {}", a, b);
        });

    return Ok(());
}
