// 8 for 256 bit initeger
// 5 for 160 bit integer
// 4 for 128 bit integer
const WORDS: usize = 8;

use std::ops::{Add, Sub};

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
		for i in 0..WORDS-2 {
			result.words[i] = words[i+1];
		}
		result
	}

	fn new_from_i32(val: i32) -> iCustomSize {
		let mut result = iCustomSize::new();
		result.hi = (val >> 31)	;
		result.words[WORDS-2] = val;
		result
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
			let added: i64 = self.words[WORDS-2-i] as i64 + other.words[WORDS-2-i] as i64 + overflow as i64;
			overflow = (added >> 31) as i32;
			words[WORDS-2-i] = added as i32;
		}

		hi = hi + overflow;

        iCustomSize { hi: hi, words: words }
    }
}

impl Add<i32> for iCustomSize {
	type Output = iCustomSize;
	fn add(self, other: i32) -> iCustomSize {
		let custom_other = iCustomSize::new_from_i32(other);
		self + custom_other
	}
}

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
			overflow = (added >> 32) as u32;
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
		0 => { return; }
		_ => { panic!("newly defined int hi word is not initialized to default value of 0 (zero)");}
	}
}

#[test]
fn can_create_uCustomSize() {
	let defined_uCustomSize = uCustomSize::new();

    match defined_uCustomSize.words[0] {
		0 => { return; }
		_ => { panic!("newly defined int hi word is not initialized to default value of 0 (zero)");}
	}
}

#[test]
fn can_create_iCustomSize_from_i32() {
	let defined_iCustomSize = iCustomSize::new_from_i32(-10);
	match defined_iCustomSize.hi {
		-1 => { return; }
		_ => { panic!("hi word of the newly defined int is not negative"); }
	}

	match defined_iCustomSize.words[WORDS-1] {
		-10 => { return; }
		_ => { panic!("lo word of the newly defined int is not 10 defined on the first place"); }
	}
}

#[test]
fn can_create_iCustomSize_from_words() {
	let mut words: [i32; WORDS] = [0;WORDS];
	words[WORDS-1] = 1;
	let isz = iCustomSize::new_from_words(words);
	match isz.hi {
		0 => { return; }
        _ => { panic!("newly defined int hi word is not initialized to default value of 0 (zero)"); }
	}

	match isz.words[WORDS-1] {
		1 => { return; }
        _ => { panic!("newly defined int lowest word is not initialized to the default value of 1 (one)"); }
	}
}

#[test]
fn can_add_iCustomSize_simple() {
	let isz1 = iCustomSize::new_from_i32(10);
	let isz2 = iCustomSize::new_from_i32(20);

	let summ = isz1 + isz2;

	match summ.hi {
		0 => { return; }
		_ => { panic!("newly defined int hi word is not initialized to default value of 0 (zero)"); }
	}

	match summ.words[WORDS-1] {
		30 => { return; }
        _ => { panic!("summ of non-overflowed ints should is not exactly 30"); }
	}
}

#[test]
fn can_add_iCustomSize_negative() {
	let isz1 = iCustomSize::new_from_i32(10);
	let isz2 = iCustomSize::new_from_i32(-20);

	let summ = isz1 + isz2;

	match summ.hi {
		-1 => { return; }
		_ => { panic!("resulting int is not negative!"); }
	}

	match summ.words[WORDS-1] {
		-10 => { return; }
        _ => { panic!("result of 10 + (-20) is not exactly -10"); }
	}
}

#[test]
fn can_add_iCustomSize_with_i32() {
	let isz1 = iCustomSize::new_from_i32(-20);
	let i32v: i32 = 10;

	let summ = isz1 + i32v;

	match summ.hi {
		-1 => { return; }
		_ => { panic!("resulting int is not negative!"); }
	}

	match summ.words[WORDS-1] {
		-10 => { return; }
        _ => { panic!("result of (10) + (-20) is not exactly -10"); }
	}
}

#[test]
fn can_add_iCustomSize_negative_all() {
	let isz1 = iCustomSize::new_from_i32(-10);
	let isz2 = iCustomSize::new_from_i32(-15);

	let summ = isz1 + isz2;

	match summ.words[WORDS-2] {
		-25 => { return; }
        _ => { panic!("result of -10 + (-150) is not exactly -25"); }
	}

	match summ.hi {
		-1 => { return; }
		_ => { panic!("resulting int is not negative!"); }
	}
}

#[test]
fn can_add_uCustomSize_numbers() {
	let usz1 = uCustomSize::new_from_u32(2000000000);
	let usz2 = uCustomSize::new_from_u32(2000000000);

	let summ = usz1 + usz2;

	match summ.words[WORDS-1] {
		0 => { panic!("impossible value, maybe overflown") }
		_ => { return; }
	}

	match summ.words[WORDS-2] {
		0 => { panic!("Should be greater than because 2^9 + 2^9 is bigger than max int32"); }
		_ => { return; }
	}
}