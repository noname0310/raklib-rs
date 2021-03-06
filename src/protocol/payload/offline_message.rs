use crate::protocol::{Encode, Decode};
use bytes::BufMut;
use std::io::Read;

#[derive(Default, Debug)]
pub struct OfflineMessage {
	pub magic: [u8; 16]
}

impl OfflineMessage {
	const MAGIC: [u8; 16] = [0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78];

	fn is_valid(&self) -> bool {
		self.magic == Self::MAGIC
	}
}

impl Encode for OfflineMessage {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_slice(&Self::MAGIC);
	}
}

impl Decode for OfflineMessage {
	fn decode(serializer: &mut &[u8]) -> Self {
		let mut magic = [0; 16];
		serializer.read(&mut magic);
		Self {
			magic
		}
	}
}