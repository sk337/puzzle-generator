// randomness macros made by CasualX
//
// Licesned under MIT License
// Copyright (c) 2019-2020 Casper <CasualX@users.noreply.github.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#[macro_export]
macro_rules! hash {
	($s:expr) => {{ const _DJB2_HASH: u32 = $crate::hash($s); _DJB2_HASH }};
}

#[inline(always)]
pub const fn hash(s: &str) -> u32 {
	let s = s.as_bytes();
	let mut result = 3581u32;
	let mut i = 0usize;
	while i < s.len() {
		result = result.wrapping_mul(33) ^ s[i] as u32;
		i += 1;
	}
	return result;
}

pub const SEED: u64 = splitmix(hash(match option_env!("OBFSTR_SEED") { Some(seed) => seed, None => "FIXED" }) as u64);

#[inline(always)]
pub const fn splitmix(seed: u64) -> u64 {
	let next = seed.wrapping_add(0x9e3779b97f4a7c15);
	let mut z = next;
	z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
	z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
	return z ^ (z >> 31);
}

#[inline(always)]
pub const fn entropy(string: &str) -> u64 {
	splitmix(SEED ^ splitmix(hash(string) as u64))
}

#[macro_export]
macro_rules! random {
	($ty:ident $(, $seeds:expr)* $(,)?) => {{
		const _RANDOM: $ty = $crate::__random_cast!($ty,
			$crate::entropy(concat!(file!(), ":", line!(), ":", column!() $(, ":", $seeds)*)));
		_RANDOM
	}};
}

#[macro_export]
macro_rules! __random_cast {
	(u8, $seed:expr) => { $seed as u8 };
	(u16, $seed:expr) => { $seed as u16 };
	(u32, $seed:expr) => { $seed as u32 };
	(u64, $seed:expr) => { $seed };
	(usize, $seed:expr) => { $seed as usize };
	(i8, $seed:expr) => { $seed as i8 };
	(i16, $seed:expr) => { $seed as i16 };
	(i32, $seed:expr) => { $seed as i32 };
	(i64, $seed:expr) => { $seed as i64 };
	(isize, $seed:expr) => { $seed as isize };
	(bool, $seed:expr) => { $seed as i64 >= 0 };

	// {f32, f64}::from_bits is unstable as const fn due to issues with NaN
	(f32, $seed:expr) => { unsafe { ::core::mem::transmute::<u32, f32>(0b0_01111111 << (f32::MANTISSA_DIGITS - 1) | ($seed as u32 >> 9)) } };
	(f64, $seed:expr) => { unsafe { ::core::mem::transmute::<u64, f64>(0b0_01111111111 << (f64::MANTISSA_DIGITS - 1) | ($seed >> 12)) } };

	($ty:ident, $seed:expr) => { compile_error!(concat!("unsupported type: ", stringify!($ty))) };
}