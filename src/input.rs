use byteorder::{
	BigEndian,
	ByteOrder,
	LittleEndian,
};
use thiserror::Error;

macro_rules! def {
	(
		type = $ty:ty;
		$read:ident;
		$try_read:ident;
		$read_be:ident;
		$try_read_be:ident;
		$read_le:ident;
		$try_read_le:ident;

		|$var:ident|
		$body:tt
	) => {
		#[inline]
		pub fn $read<BO: ByteOrder>(&mut self) -> $ty {
			self.$try_read::<BO>().unwrap()
		}

		#[inline]
		pub fn $try_read<BO: ByteOrder>(&mut $var) -> Result<$ty, InputError> {
			$body
		}

		#[inline]
		pub fn $read_be(&mut self) -> $ty {
			self.$read::<BigEndian>()
		}

		#[inline]
		pub fn $try_read_be(&mut self) -> Result<$ty, InputError> {
			self.$try_read::<BigEndian>()
		}

		#[inline]
		pub fn $read_le(&mut self) -> $ty {
			self.$read::<LittleEndian>()
		}

		#[inline]
		pub fn $try_read_le(&mut self) -> Result<$ty, InputError> {
			self.$try_read::<LittleEndian>()
		}
	};
}

#[derive(Default, Copy, Clone)]
#[cfg(feature = "ctx")]
pub struct Context {
	pub position: usize,
}

#[derive(Copy, Clone)]
#[cfg_attr(not(feature = "ctx"), repr(transparent))]
pub struct Input<'a>(
	&'a [u8],
	#[cfg(feature = "ctx")]
	Context,
);

#[derive(Debug, Error)]
pub enum InputError {
	#[error("Requires at least {0} more bytes to continue.")]
	NeedsMoreData(usize)
}

impl<'a> Input<'a> {
	pub fn new(input: &[u8]) -> Input {
		Input(
			input,
			#[cfg(feature = "ctx")]
				Context::default(),
		)
	}

	#[cfg(feature = "ctx")]
	pub fn context(&self) -> &Context {
		&self.1
	}

	pub fn remaining(&self) -> usize {
		self.0.len()
	}

	pub fn read_slice(&mut self, count: usize) -> &'a [u8] {
		self.try_read_slice(count).unwrap()
	}

	pub fn try_read_slice(&mut self, count: usize) -> Result<&'a [u8], InputError> {
		let remaining = self.remaining();
		if remaining < count {
			return Err(InputError::NeedsMoreData(count - remaining));
		}
		let (data, remaining) = self.0.split_at(count);
		self.0 = remaining;
		#[cfg(feature = "ctx")]
		{
			self.1.position += count;
		}
		Ok(data)
	}

	pub fn read_u8(&mut self) -> u8 {
		self.try_read_u8().unwrap()
	}

	pub fn try_read_u8(&mut self) -> Result<u8, InputError> {
		let data = self.try_read_slice(1)?;
		Ok(data[0])
	}

	pub fn read_i8(&mut self) -> i8 {
		self.try_read_i8().unwrap()
	}

	pub fn try_read_i8(&mut self) -> Result<i8, InputError> {
		let data = self.try_read_slice(1)?;
		Ok(data[0] as i8)
	}

	def! {
		type = u16;
		read_u16;
		try_read_u16;
		read_u16_be;
		try_read_u16_be;
		read_u16_le;
		try_read_u16_le;

		|self| {
			let data = self.try_read_slice(2)?;
			Ok(BO::read_u16(data))
		}
	}

	def! {
		type = i16;
		read_i16;
		try_read_i16;
		read_i16_be;
		try_read_i16_be;
		read_i16_le;
		try_read_i16_le;

		|self| {
			let data = self.try_read_slice(2)?;
			Ok(BO::read_i16(data))
		}
	}

	def! {
		type = u32;
		read_u32;
		try_read_u32;
		read_u32_be;
		try_read_u32_be;
		read_u32_le;
		try_read_u32_le;

		|self| {
			let data = self.try_read_slice(4)?;
			Ok(BO::read_u32(data))
		}
	}

	def! {
		type = i32;
		read_i32;
		try_read_i32;
		read_i32_be;
		try_read_i32_be;
		read_i32_le;
		try_read_i32_le;

		|self| {
			let data = self.try_read_slice(4)?;
			Ok(BO::read_i32(data))
		}
	}

	def! {
		type = u64;
		read_u64;
		try_read_u64;
		read_u64_be;
		try_read_u64_be;
		read_u64_le;
		try_read_u64_le;

		|self| {
			let data = self.try_read_slice(8)?;
			Ok(BO::read_u64(data))
		}
	}

	def! {
		type = i64;
		read_i64;
		try_read_i64;
		read_i64_be;
		try_read_i64_be;
		read_i64_le;
		try_read_i64_le;

		|self| {
			let data = self.try_read_slice(8)?;
			Ok(BO::read_i64(data))
		}
	}

	#[cfg(has_u128)]
	def! {
		type = u128;
		read_u128;
		try_read_u128;
		read_u128_be;
		try_read_u128_be;
		read_u128_le;
		try_read_u128_le;

		|self| {
			let data = self.try_read_slice(16)?;
			Ok(BO::read_u128(data))
		}
	}

	#[cfg(has_i128)]
	def! {
		type = i128;
		read_i128;
		try_read_i128;
		read_i128_be;
		try_read_i128_be;
		read_i128_le;
		try_read_i128_le;

		|self| {
			let data = self.try_read_slice(16)?;
			Ok(BO::read_i128(data))
		}
	}

	def! {
		type = f32;
		read_f32;
		try_read_f32;
		read_f32_be;
		try_read_f32_be;
		read_f32_le;
		try_read_f32_le;

		|self| {
			let data = self.try_read_slice(4)?;
			Ok(BO::read_f32(data))
		}
	}

	def! {
		type = f64;
		read_f64;
		try_read_f64;
		read_f64_be;
		try_read_f64_be;
		read_f64_le;
		try_read_f64_le;

		|self| {
			let data = self.try_read_slice(4)?;
			Ok(BO::read_f64(data))
		}
	}
}
