pub mod classes;
pub mod types;
use std::env::args;

/// Exercices Includes
pub mod ex00;
pub mod ex01;
pub mod ex02;
pub mod ex03;
pub mod ex04;
pub mod ex05;
pub mod ex06;
pub mod ex07;
pub mod ex08;
pub mod ex09;
pub mod ex10;
pub mod ex11;
pub mod ex12;
pub mod ex13;
pub mod ex14;
pub mod ex15;
use ex00::test::ex00;
use ex01::test::ex01;
use ex02::test::ex02;
use ex03::test::ex03;
use ex04::test::ex04;
use ex05::test::ex05;
use ex06::test::ex06;
use ex07::test::ex07;
use ex08::test::ex08;
use ex09::test::ex09;
use ex10::test::ex10;
use ex11::test::ex11;
use ex12::test::ex12;
use ex13::test::ex13;
use ex14::test::ex14;
use ex15::test::ex15;

fn main() {
	let args: Vec<String> = args().collect();
	if args.len() != 2 
	{
		println!("You must put the exercice number as an arg (from 0 to 15) to run the tests");
		return;
	}
	let exercice : i32 = args[1].trim().parse().expect("you must give a number between 0 and 15"); 
	match exercice {
		0 => ex00(),
		1 => ex01(),
		2 => ex02(),
		3 => ex03(),
		4 => ex04(), 
		5 => ex05(),
		6 => ex06(),
		7 => ex07(),
		8 => ex08(),
		9 => ex09(),
		10 => ex10(),
		11 => ex11(),
		12 => ex12(),
		13 => ex13(),
		14 => ex14().unwrap(),
		15 => ex15(),
		_ => {
			println!("you must give a number between 0 and 15");
		}
	}
}
