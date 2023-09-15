use std::str::FromStr;

use shlex::Shlex;

struct Httpie {
	url: String,
	options: Options,
	headers: Headers,
	body: Body,
}

struct Options {}
struct Headers {}
struct Body;

impl FromStr for Httpie {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Shlex::new(s).for_each(|c| println!("{c}"));
		todo!()
	}
}

#[test]
fn test() {
	assert!(Httpie::from_str(
		r#"http --chunked --multipart PUT pie.dev/put hello=world foo@files/data.xml"#,
	)
	.is_ok());
}
