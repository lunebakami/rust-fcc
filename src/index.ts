// Exercise 1 - Create a list and add 1 to each item
// const list = [1, 2, 3].map(item => item + 1)
// console.log(list)

// Exercise 2 - Read lines from file
// import fs from 'fs'

// fs.readFileSync('./lines').
//   toString().
//   split("\n").
//   filter((_, i) => i % 2 === 0).
//   filter((_, i) => i > 1 && i < 4).
//   forEach(line => console.log(line))

// Exercise 3 - Enums
// enum Color {
//   Red,
//   Green,
//   Blue
// }

// function printColor(color: Color) {
//   switch(color) {
//     case Color.Red:
//       console.log("red");
//       break;
//     case Color.Blue:
//       console.log("blue");
//       break;
//     case Color.Green: 
//       console.log("green");
//       break;
//   }
// }

// printColor(Color.Red)

// type Custom = {
//   age: number;
//   name: string;
// }

// type Item = number | string | Custom;

// function append(items: Item[]) {
//   items.push("Hello Fem!")
// }

// const arr: Item[] = []

// append(arr);

// console.log(arr)

// function foo(val: number | undefined) {
//   if (typeof val === 'number') {
//     return val * 5;
//   }

//   if (typeof val === 'undefined') {
//     return 0;
//   }
// }

// import fs from 'fs';

// const filePath = process.argv[2];

// try {
//   if (!filePath) {
//     console.log("erro")
//   } else {
//     const file = fs.readFileSync(filePath)

//     file.toString().split("/n").forEach(line => console.log(line))
//   }
  
// } catch (error) {
//   console.log("error")
// }

interface Area {
  area(): number;
}

class Rectangle implements Area{
  constructor(
    public x: number,
    public y: number,
    public width: number,
    public height: number
  ) {}

  area(): number {
    return this.width * this.height
  }
}

class Circle {
  constructor(
    public x: number,
    public y: number,
    public radius: number,
  ) {}

  area(): number {
    return this.radius * this.radius * Math.PI
  }
}

