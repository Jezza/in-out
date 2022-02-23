use byteorder::{
	ByteOrder,
	BigEndian,
	LittleEndian,
};

use byteorder::WriteBytesExt;

pub struct Output(Vec<u8>);

macro_rules! def {
	(
		type = $ty:ty;
		$write:ident;
		$write_be:ident;
		$write_le:ident;

		|$self:ident, $var:ident|
		$body:tt
	) => {
		pub fn $write<BO: ByteOrder>(&mut $self, $var: $ty) {
			$body
		}

		#[inline]
		pub fn $write_be(&mut self, value: $ty) {
			self.$write::<BigEndian>(value)
		}
	
		#[inline]
		pub fn $write_le(&mut self, value: $ty) {
			self.$write::<LittleEndian>(value)
		}
	};
}

impl Output {
	pub fn new() -> Output {
		Output(vec![])
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn into_inner(self) -> Vec<u8> {
		self.0
	}

	pub fn as_slice(&self) -> &[u8] {
		&self.0
	}

	pub fn write_slice(&mut self, data: &[u8]) {
		self.0.extend_from_slice(data)
	}

	#[inline]
	pub fn write_u8(&mut self, value: u8) {
		self.0.push(value);
	}

	#[inline]
	pub fn write_i8(&mut self, value: i8) {
		self.write_u8(value as u8);
	}

	def! {
		type = u16;
		write_u16;
		write_u16_be;
		write_u16_le;

		|self, value| {
			// We can safely unwrap this, we're writing it into a infallible buffer. (The internal vec)
			self.0.write_u16::<BO>(value).unwrap();
		}
	}

	def! {
		type = i16;
		write_i16;
		write_i16_be;
		write_i16_le;

		|self, value| {
			// We can safely unwrap this, we're writing it into a infallible buffer. (The internal vec)
			self.0.write_i16::<BO>(value).unwrap();
		}
	}

	def! {
		type = u32;
		write_u32;
		write_u32_be;
		write_u32_le;

		|self, value| {
			// We can safely unwrap this, we're writing it into a infallible buffer. (The internal vec)
			self.0.write_u32::<BO>(value).unwrap();
		}
	}

	def! {
		type = i32;
		write_i32;
		write_i32_be;
		write_i32_le;

		|self, value| {
			// We can safely unwrap this, we're writing it into a infallible buffer. (The internal vec)
			self.0.write_i32::<BO>(value).unwrap();
		}
	}

	def! {
		type = u64;
		write_u64;
		write_u64_be;
		write_u64_le;

		|self, value| {
			// We can safely unwrap this, we're writing it into a infallible buffer. (The internal vec)
			self.0.write_u64::<BO>(value).unwrap();
		}
	}

	def! {
		type = i64;
		write_i64;
		write_i64_be;
		write_i64_le;

		|self, value| {
			// We can safely unwrap this, we're writing it into a infallible buffer. (The internal vec)
			self.0.write_i64::<BO>(value).unwrap();
		}
	}

	#[cfg(has_u128)]
	def! {
		type = u128;
		write_u128;
		write_u128_be;
		write_u128_le;

		|self, value| {
			// We can safely unwrap this, we're writing it into a infallible buffer. (The internal vec)
			self.0.write_u128::<BO>(value).unwrap();
		}
	}

	#[cfg(has_i128)]
	def! {
		type = i128;
		write_i128;
		write_i128_be;
		write_i128_le;

		|self, value| {
			// We can safely unwrap this, we're writing it into a infallible buffer. (The internal vec)
			self.0.write_i128::<BO>(value).unwrap();
		}
	}

	def! {
		type = f32;
		write_f32;
		write_f32_be;
		write_f32_le;

		|self, value| {
			// We can safely unwrap this, we're writing it into a infallible buffer. (The internal vec)
			self.0.write_f32::<BO>(value).unwrap();
		}
	}

	def! {
		type = f64;
		write_f64;
		write_f64_be;
		write_f64_le;

		|self, value| {
			// We can safely unwrap this, we're writing it into a infallible buffer. (The internal vec)
			self.0.write_f64::<BO>(value).unwrap();
		}
	}
}