/// Given a size in bytes, return the size in bytes of the total full pages
/// required to cover the size.
pub fn size_fit_page(size: u64) -> u64 {
	match size&0xfff {
		0 => size,
		remainder => size+(0x1000-remainder),
	}
}

/// Given a size in bytes, return the size in bytes of the naturally-aligned
/// structure required to cover the size.
/// 
/// This is the smallest power of two that is equal to or higher than the input
/// size.
pub fn size_fit_natural(size: u64) -> u64 {
	use std::num::Wrapping;
	let mut v=Wrapping(size);
	v-=Wrapping(1);
	v |= v >> 1;
	v |= v >> 2;
	v |= v >> 4;
	v |= v >> 8;
	v |= v >> 16;
	v |= v >> 32;
	v+=Wrapping(1);
	v.0
}

#[test]
fn enclave_size_test() {
	let mut i=0x1000;
	loop {
		assert_eq!(enclave_size(i-1),i);
		assert_eq!(enclave_size(i),i);
		assert_eq!(enclave_size(i+1),i<<1);
		if i>0x8000_0000_0000_0000 {
			i<<=1;
		} else {
			break;
		}
	}
}
