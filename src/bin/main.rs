fn main() {
	use in_out::Input;

	// Some data source... Could be a vec, array, etc
	let data = vec![0xDE, 0xAD, 0xBE, 0xEF, 2, 0b0000_0001];

	let mut input = Input::new(&data);

	let magic = input.try_read_u32_be()
		.expect("Unable to read Magic Number...");
	let version = input.read_u8();
	let flags = input.read_u8();

	println!("0x{:X} v{} flags:{:08b}", magic, version, flags);
}