#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
// 16 for 512 bit integer
// 8 for 256 bit initeger
// 5 for 160 bit integer
// 4 for 128 bit integer
const WORDS: usize = 8;

use std::ops::{Add, Sub, Mul};
use std::str::FromStr;
use std::cmp::PartialEq;
use std::ops::Neg;

#[derive(Debug, Copy, Clone)]
struct iCustomSize {
	hi: i32,
	words: [i32; WORDS-1]
}

impl iCustomSize {
    fn new() -> iCustomSize {
		iCustomSize { hi: 0, words: [0; WORDS-1]}
	}

	fn new_from_words(words: [i32; WORDS]) -> iCustomSize {
		let mut result = iCustomSize::new();
		result.hi = words[0] as i32;
		for i in 0..WORDS-1 {
			result.words[i] = words[i+1];
		}
		result
	}

	fn new_from_i32(val: i32) -> iCustomSize {
		let mut result = iCustomSize::new();
		result.hi = val >> 31;
		result.words[WORDS-2] = val;
		for i in 0..WORDS-2 {
			result.words[i] = val >> 31;
		}
		result
	}

	fn negate(self) -> iCustomSize {
		let mut result = iCustomSize::new();
		result.hi = !self.hi + (self.hi >> 31);
		for i in 0..WORDS-1 {
			result.words[i] = match self.words[i] {
				1 => -1,
				_ => !self.words[i] + (self.words[i] >> 31)
			};
		}
		result
	}

	fn new_from_str(s: &str) -> iCustomSize {
		match iCustomSize::from_str(s) {
			Ok(sz) => sz,
			_ => panic!("Parse error!")
		}
	}
}



impl Neg for iCustomSize {
    type Output = iCustomSize;

    fn neg(self) -> iCustomSize {
        self.negate()
    }
}

impl Add for iCustomSize {
    type Output = iCustomSize;
    fn add(self, other: iCustomSize) -> iCustomSize {
		let mut words: [i32; WORDS-1] = [0;WORDS-1];
    	let mut overflow: i32 = 0;

		let self_carry = self.hi >> 31;
		let other_carry = other.hi >> 31;
		// including carry bits for negative numbers
		let mut hi: i32 = self.hi + other.hi - self_carry - other_carry;

		for i in 0..WORDS-1 {
			let index = WORDS-2-i;
			let carry = match i {
				0 => (0, 0),
				_ => (self.words[index+1] >> 31, other.words[index+1] >> 31)
			};

			let added: i64 =
				self.words[index] as i64
				+ other.words[index] as i64
				+ overflow as i64
				- carry.0 as i64
				- carry.1 as i64;

			let mut word_value = 0;
			if (added >= 0) {
				word_value = added as i32;
			}
			else {
				word_value = (added | (-1 << 31)) as i32;
			}

			overflow = (added >> 31) as i32;
			words[index] = word_value as i32;
		}

		hi = hi + overflow;

        iCustomSize { hi: hi, words: words }
    }
}

impl Sub for iCustomSize {
	type Output = iCustomSize;
	fn sub(self, other: iCustomSize) -> iCustomSize {
		self + other.negate()
	}
}

impl Add<i32> for iCustomSize {
	type Output = iCustomSize;
	fn add(self, other: i32) -> iCustomSize {
		let custom_other = iCustomSize::new_from_i32(other);
		self + custom_other
	}
}

impl Mul for iCustomSize {
	type Output = iCustomSize;

	fn mul(self, other: iCustomSize) -> iCustomSize {
		let mut words: [i32; WORDS-1] = [0;WORDS-1];
    	let mut overflow: i32 = 1;

		for i in 0..WORDS-1 {
			let index = WORDS-2-i;

			let s1 = match (self.words[index], i)
			{
				(0, 0) => 0 as i64,
				(0, _) => (overflow >> 31) as i64,
				(_, _) => self.words[index] as i64
			};

			let s2 = match(other.words[index], i)
			{
				(0, 0) => 0 as i64,
				(0, _) => (overflow >> 31) as i64,
				(_, _) => other.words[index] as i64
			};

			let product:i64 =
				s1
				* s2
				* overflow as i64;

			println!("{} * {} * {} = {}", s1, s2, overflow as i64, product);

			overflow = (product >> 31) as i32;

			let mut word_value = 0;
			if (product >= 0) {
				word_value = product as i32;
			}
			else {
				word_value = (product | (-1 << 31)) as i32;
			}

			words[index] = word_value;

		}

		let hi1 = match self.hi {
			0 => overflow >> 31,
			_ => self.hi
		};

		let hi2 = match other.hi {
			0 => overflow >> 31,
			_ => other.hi
		};

		let hi = hi1 * hi2 * overflow;
		println!("{} * {} * {} = {}", hi1, hi2, overflow as i64, hi);

		iCustomSize { hi: hi, words: words }
	}
}

impl Mul<i32> for iCustomSize {
	type Output = iCustomSize;
	fn mul(self, other: i32) -> iCustomSize {
		let custom_other = iCustomSize::new_from_i32(other);
		self * custom_other
	}
}

impl PartialEq<iCustomSize> for iCustomSize {
	fn eq(&self, other: &iCustomSize) -> bool {
		if (self.hi != other.hi) {
			return false;
		}

		for i in 0..WORDS-2 {
			if (self.words[i] != other.words[i]) {
				return false;
			}
		}
		true
	}
}

enum iCustomSizeError {
	OverflowError,
	FormatError
}

impl FromStr for iCustomSize {
    type Err = iCustomSizeError;
    fn from_str(s: &str) -> Result<iCustomSize, iCustomSizeError> {
		let first_char = s.chars().nth(0);
		let is_negative = (first_char == Some('-'));

		print!("is_negative: {}", is_negative);

		let numbers_string:Vec<u8> = match is_negative {
			true => s.bytes().skip(1).collect(),
			false => s.bytes().collect()
		};

		let mut result: iCustomSize = iCustomSize::new_from_i32(0);
		let mut base = iCustomSize::new_from_i32(1);
		for c in numbers_string {
			let digit = (c - 48) as i32;
			result = match is_negative {
				false => result + base * digit,
				true => result - base * digit
			};
			base = base * 10;
		}
		Ok(result)
	}
}


#[derive(Debug, Copy, Clone)]
struct uCustomSize {
	words: [u32; WORDS]
}

impl uCustomSize {
    fn new() -> uCustomSize {
		uCustomSize { words: [0; WORDS] }
	}

	fn new_from_u32(val: u32) -> uCustomSize {
		let mut result = uCustomSize::new();
		result.words[WORDS-1] = val;
		result
	}
}

impl Add for uCustomSize {
    type Output = uCustomSize;
    fn add(self, other: uCustomSize) -> uCustomSize {
		let mut words: [u32; WORDS] = [0;WORDS];
    	let mut overflow: u32 = 0;
		for i in 0..WORDS-1 {
			let index = WORDS-1-i;
			let added: u64 = self.words[index] as u64 + other.words[index] as u64 + overflow as u64;
			overflow = (added >> 31) as u32;
			words[index] = added as u32;
		}
        uCustomSize { words: words }
    }
}

impl Add<u32> for uCustomSize {
	type Output = uCustomSize;
	fn add(self, other: u32) -> uCustomSize {
		let custom_other = uCustomSize::new_from_u32(other);
		self + custom_other
	}
}


#[test]
fn can_create_iCustomSize() {
	let defined_iCustomSize = iCustomSize::new();

    match defined_iCustomSize.hi {
		0 => { }
		_ => { panic!("newly defined int hi word is not initialized to default value of 0 (zero)");}
	}
}

#[test]
fn can_create_uCustomSize() {
	let defined_uCustomSize = uCustomSize::new();

    match defined_uCustomSize.words[0] {
		0 => { }
		_ => { panic!("newly defined int hi word is not initialized to default value of 0 (zero)");}
	}
}

#[test]
fn can_create_iCustomSize_from_i32() {
	let defined_iCustomSize = iCustomSize::new_from_i32(-10);
	match defined_iCustomSize.hi {
		-1 => {}
		_ => { panic!("hi word of the newly defined int is not negative"); }
	}

	match defined_iCustomSize.words[WORDS-2] {
		-10 => { }
		_ => { panic!("lo word of the newly defined int is not 10 defined on the first place"); }
	}
}

#[test]
fn can_create_iCustomSize_from_words() {
	let mut words: [i32; WORDS] = [0;WORDS];
	words[WORDS-1] = 1;
	let isz = iCustomSize::new_from_words(words);
	match isz.hi {
		0 => { }
        _ => { panic!("newly defined int hi word is not initialized to default value of 0 (zero)"); }
	}

	match isz.words[WORDS-2] {
		1 => { }
        _ => { panic!("newly defined int lowest word is not initialized to the default value of 1 (one)"); }
	}
}

#[test]
fn can_add_iCustomSize_simple() {
	let isz1 = iCustomSize::new_from_i32(10);
	let isz2 = iCustomSize::new_from_i32(20);

	let summ = isz1 + isz2;

	match summ.hi {
		0 => { }
		_ => { panic!("newly defined int hi word is not initialized to default value of 0 (zero)"); }
	}

	match summ.words[WORDS-2] {
		30 => { }
        _ => { panic!("summ of non-overflowed ints should is not exactly 30"); }
	}
}


#[test]
fn can_add_iCustomSize_negative() {
	let isz1 = iCustomSize::new_from_i32(10);
	let isz2 = iCustomSize::new_from_i32(-20);

	let summ = isz1 + isz2;

	match summ.hi {
		-1 => { }
		_ => { panic!("resulting int is not negative!"); }
	}

	match summ.words[WORDS-2] {
		-10 => { }
        _ => { panic!("result of 10 + (-20) is not exactly -10"); }
	}
}


#[test]
fn can_add_iCustomSize_with_i32() {
	let isz1 = iCustomSize::new_from_i32(-20);
	let i32v: i32 = 10;

	let summ = isz1 + i32v;

	match summ.hi {
		-1 => { }
		_ => { panic!("resulting int is not negative!"); }
	}

	match summ.words[WORDS-2] {
		-10 => { }
        _ => { panic!("result of (10) + (-20) is not exactly -10"); }
	}
}


#[test]
fn can_add_iCustomSize_negative_all() {
	let isz1 = iCustomSize::new_from_i32(-10);
	let isz2 = iCustomSize::new_from_i32(-15);

	let summ = isz1 + isz2;

	print!("words: ");
	for x in &summ.words {
		print!("{} :", x);
	}
	println!("");

	match summ.words[WORDS-2] {
		-25 => { }
        _ => { panic!("result of -10 + (-15) is not exactly -25"); }
	}

	match summ.hi {
		-1 => { }
		_ => { panic!("resulting int is not negative!"); }
	}
}

#[test]
fn can_substract_iCustomSize_numbers() {
	let isz1 = iCustomSize::new_from_i32(-2000000000);
	let isz2 = iCustomSize::new_from_i32(2000000000);

	let subs = isz1 - isz2;

	if (subs.words[WORDS-2] >= 0)
	{
		print!("words: ");
		for x in &subs.words {
			print!("{} :", x);
		}
		println!("");

		panic!("result lowest word should be less than zero (sinze -2^9 - 2^9 is surely are less than zero)")
	}

	if (subs.words[WORDS-3] >= 0) {
		panic!("result second lowest word should be less than zero (sinze -2^9 - 2^9 is surely are less than (-max_int))")
	}

	if (subs.hi != -1) {
		println!("subs.hi = {}", subs.hi);

		panic!("the resulting number highest word should be exactiy (-1) in two-bits complement");
	}
}


#[test]
fn can_add_uCustomSize_numbers() {
	let usz1 = uCustomSize::new_from_u32(2000000000);
	let usz2 = uCustomSize::new_from_u32(2000000000);

	let summ = usz1 + usz2;

	match summ.words[WORDS-1] {
		0 => { panic!("impossible value, maybe overflown") }
		_ => { }
	}

	match summ.words[WORDS-2] {
		0 => { panic!("Should be greater than because 2^9 + 2^9 is bigger than max int32"); }
		_ => { }
	}
}


#[test]
fn can_add_iCustomSize_big_numbers() {
	let isz1 = iCustomSize::new_from_i32(-2000000000);
	let isz2 = iCustomSize::new_from_i32(-2000000000);

	let summ = isz1 + isz2;

	if (summ.words[WORDS-2] >= 0) {
		panic!("result lowest word should be less than zero (sinze -2^9 + (-2^9) is surely are less than zero)")
	}
}

#[test]
fn can_multiply_iCustomSize_simple() {
	let isz1 = iCustomSize::new_from_i32(2);
	let isz2 = iCustomSize::new_from_i32(2);

	let product = isz1 * isz2;
	match product.hi {
		0 => {},
		_ => { panic!("hi word of the product should be exactly 0 since 2*2 is definitely not overflown")}
	}

	match product.words[WORDS-2] {
		4 => {},
		_ => { panic!("2*2 is not 4")}
	}
}

#[test]
fn can_multiply_iCustomSize_negative() {
	let isz1 = iCustomSize::new_from_i32(-2);
	let isz2 = iCustomSize::new_from_i32(-2);

	let product = isz1 * isz2;
	match product.hi {
		0 => {},
		_ => { panic!("hi word of the product should be exactly 0 since -2*-2 and the product is positive")}
	}
}

#[test]
fn can_multiply_iCustomSize_opposite() {
	let isz1 = iCustomSize::new_from_i32(-2);
	let isz2 = iCustomSize::new_from_i32(2);

	let product = isz1 * isz2;
	match product.hi {
		-1 => {},
		_ => { panic!("hi word of the product should be exactly -1 since we have little negative 4 as a product")}
	}

	let product2 = isz2 * isz1;
	match product2.hi {
		-1 => {},
		_ => { panic!("hi word of the product should be exactly -1 since we have little negative 4 as a product")}
	}
}

#[test]
fn can_create_iCustomSize_from_string() {
	let isz_result = iCustomSize::from_str("1");

	match isz_result {
		Ok(isz) => match (isz.hi, isz.words[WORDS-2])  {
			(0, 1) => {},
			(_, _) => { panic!("hi word of 1 (one) should be 0 (zero), lowest word should be 1 (one)");}
		},
		_ => { panic!("error parsing VERY simple string");}
	}
}

#[test]
fn can_create_negative_iCustomSize_from_string() {
	let isz_result = iCustomSize::from_str("-1");

	match isz_result {
		Ok(isz) => match (isz.hi, isz.words[WORDS-2])  {
			(-1, -1) => {},
			(_, _) => {
				println!("hi: {}, lo: {}", isz.hi, isz.words[WORDS-2]);
				panic!("hi word of -1 (negative one) should be -1 (zero), lowest word should be -1 (negative one)");
			}
		},
		_ => { panic!("error parsing VERY simple string");}
	}
}

#[test]
fn can_substract_iCustomSize_from_zero() {
	let isz1 = iCustomSize::new_from_i32(0);
	let isz2 = iCustomSize::new_from_i32(-1);

	let sub = isz1 - isz2;

	match (sub.hi, sub.words[WORDS-2])  {
		(-1, -1) => {},
		(_, _) => { panic!("hi word of -1 (negative one) should be -1 (zero), lowest word should be -1 (negative one)");}
	}
}

#[test]
fn can_pass_wau_tests() {
	let isz1 = iCustomSize::new_from_i32(1);
	let isz2 = iCustomSize::new_from_i32(1);
	let product1 = isz1*isz2;

	match (product1.hi, product1.words[WORDS-2])  {
		(0, 1) => {},
		(_, _) => { panic!("hi word of 1 (one) should be 0 (zero), lowest word should be 1 (one)");}
	}

	let product2 = isz1*(-isz2);
	match (product2.hi, product2.words[WORDS-2])  {
		(-1, -1) => {},
		(_, _) => { panic!("hi word of -1 (negative one) should be -1 (zero), lowest word should be -1 (negative one)");}
	}
}

#[test]
fn can_negate_iCustomSize() {
	let isz = iCustomSize::new_from_i32(1);
	let isz_negated = isz.negate();

	match (isz_negated.hi, isz_negated.words[WORDS-2])  {
		(-1, -1) => {},
		(_, _) => { panic!("hi word of -1 (negative one) should be -1 (zero), lowest word should be -1 (negative one)");}
	}
}

#[test]
fn can_multiply_really_big_numbers() {
	let isz1 = iCustomSize::new_from_str("1000000000000000");
	let isz2 = iCustomSize::new_from_str("2000000000000000");
	let product = isz1 * isz2;
	let product_test = iCustomSize::new_from_str("2000000000000000000000000000000");

	assert_eq!(product, product_test);
}

#[test]
fn can_multiply_really_big_negative_numbers() {
	let isz1 = iCustomSize::new_from_str("-1000000000000000");
	let isz2 = iCustomSize::new_from_str("-2000000000000000");
	let product = isz1 * isz2;
	let product_test = iCustomSize::new_from_str("2000000000000000000000000000000");

	assert_eq!(product, product_test);
}

#[test]
fn can_multiply_really_big_opposite_numbers() {
	let isz1 = iCustomSize::new_from_str("-3000000000000000");
	let isz2 = iCustomSize::new_from_str("2000000000000000");
	let product = isz1 * isz2;
	let product_test = iCustomSize::new_from_str("-6000000000000000000000000000000");

	assert_eq!(product, product_test);
}

#[test]
fn can_compare_iCustomSize() {
	let isz1 = iCustomSize::new_from_i32(100);
	let isz2 = iCustomSize::new_from_i32(100);

	if (isz1 != isz2) {
		panic!("100 is actually pretty equal to 100");
	}
}

//#[test]
//fn shl() {
//	let x:i64 = -4000000000;
//	let y:i32 = (x >> 32) as i32;
//	let r:i32 = (x | (-1 << 31)) as i32;
//	println!("y: {}, r: {}", y, r);
//
//	panic!();
//}
