pub fn get() -> u128 {
	let d = [0, 0, 32, 59, 157, 181, 5, 111, 0, 0, 0, 0, 0, 0, 0, 0];
	decode_impl(&mut &d[..]).unwrap()
}

#[no_mangle]
fn main() {
	let c: u128 = get();
	assert_eq!(c, 8000000000000000000u128);
}

#[inline(never)]
fn decode_impl(input: &mut &[u8]) -> Result<u128, ()> {
	let mut buf = [0u8; 16];
	read(input, &mut buf)?;
	Ok(<u128>::from_le_bytes(buf))
}

#[inline(never)]
fn read(input: &mut &[u8], into: &mut [u8]) -> Result<(), ()> {
	if into.len() != input.len() {
		return Err(());
	}
	into.copy_from_slice(&input[..]);
	Ok(())
}
