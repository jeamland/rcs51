use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::num;

#[derive(Debug)]
struct NoSuchRecordTypeError {
	code: u8,
	desc: String,
}

impl error::Error for NoSuchRecordTypeError {
	fn description(&self) -> &str {
		self.desc.as_str()
	}
}

impl fmt::Display for NoSuchRecordTypeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.desc)
	}
}

#[derive(Debug)]
enum IntelHexParseError {
	Io(io::Error),
	ParseInt(num::ParseIntError),
	ParseRecordType(NoSuchRecordTypeError),
}

impl From<io::Error> for IntelHexParseError {
	fn from(err: io::Error) -> IntelHexParseError {
		IntelHexParseError::Io(err)
	}
}

impl From<num::ParseIntError> for IntelHexParseError {
	fn from(err: num::ParseIntError) -> IntelHexParseError {
		IntelHexParseError::ParseInt(err)
	}
}

impl From<NoSuchRecordTypeError> for IntelHexParseError {
	fn from(err: NoSuchRecordTypeError) -> IntelHexParseError {
		IntelHexParseError::ParseRecordType(err)
	}
}

#[derive(Debug)]
enum IntelHexRecordType {
	Data,
	EndOfFile,
}

impl IntelHexRecordType {
	pub fn parse(src: &str) -> Result<IntelHexRecordType, IntelHexParseError> {
		let code = u8::from_str_radix(src, 16)?;
		match code {
			0 => Ok(IntelHexRecordType::Data),
			1 => Ok(IntelHexRecordType::EndOfFile),
			_ => Err(IntelHexParseError::ParseRecordType(NoSuchRecordTypeError { code: code, desc: format!("unknown code: {}", code) }))
		}
	}
}

#[derive(Debug)]
struct IntelHexRecord {
	byte_count: u8,
	address: u16,
	record_type: IntelHexRecordType,
	data: Vec<u8>,
	checksum: u8,
}

impl IntelHexRecord {
	pub fn parse(src: &str) -> Result<IntelHexRecord, IntelHexParseError> {
		if &src[0..1] != ":" {
			panic!("bad start byte");
		}
	
		let byte_count = u8::from_str_radix(&src[1..3], 16)?;
		let address = u16::from_str_radix(&src[3..7], 16)?;
		let record_type = IntelHexRecordType::parse(&src[7..9])?;
		let data_end = 9 + (byte_count * 2) as usize;
		let checksum = u8::from_str_radix(&src[data_end..(data_end + 2)], 16)?;

		let mut data = Vec::with_capacity(byte_count as usize);
		for byte in 1..byte_count {
			let pos = 9 + (byte * 2) as usize;
			data.push(u8::from_str_radix(&src[pos..(pos + 2)], 16)?);
		}

		Ok(IntelHexRecord {
			byte_count: byte_count,
			address: address,
			record_type: record_type,
			data: data,
			checksum: checksum
		})
	}
}

fn main() {
    let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		panic!("must provide filename");
	}

	let filename = &args[1];
    let file = fs::File::open(filename).expect("file not found");
	let reader = io::BufReader::new(file);

	for line in reader.lines() {
		match line {
			Ok(line) => {
				let record = IntelHexRecord::parse(line.as_str());
				println!("{:?}", record);
			}
			Err(_e) => {
				panic!("omgwtfbbq");
			}
		}
	}
}