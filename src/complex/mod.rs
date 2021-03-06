
//! Contains complex avro types declaration such as Records etc.

mod enum_type;
pub use self::enum_type::Enum;
mod record_type;
pub use self::record_type::{Record, Field};

// lazy_static! {
//     static ref NAME_MATCHER: Regex = Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*").unwrap();
// }

/// Represents `fullname` attribute of a named avro type
#[derive(Debug, PartialEq, Clone)]
pub struct Named {
	name: String,
	namespace: Option<String>,
	doc: Option<String>
}

impl Named {
	fn new(name: &str, namespace: Option<String>, doc: Option<String>) -> Self {
		Named {
			name: name.to_string(),
			doc: doc,
			namespace: namespace
		}
	}
}

// 	fn get_name(&self) -> &str {
// 		self.name.as_str()	
// 	}

// 	fn get_namespace(&self) -> Option<&String> {
// 		self.namespace.as_ref()
// 	}

// 	fn validate(&self) -> Result<(), AvroErr> {
// 		if !NAME_MATCHER.is_match(&self.name) {
// 			return Err(AvroErr::InvalidFullname);
// 		} else if self.namespace.as_ref().map(|c|c.contains(".")).unwrap_or(false) {
// 			let names = self.namespace.as_ref().map(|s| s.split(".")).unwrap();
// 			for n in names {
// 				if !NAME_MATCHER.is_match(n) {
// 					return Err(AvroErr::InvalidFullname);
// 				}
// 			}
// 			return Ok(());
// 		} else {
// 			Err(AvroErr::InvalidFullname)
// 		}
// 	}

// 	/// Retrieves the fullname of the corresponding named type
// 	pub fn fullname(&self) -> String {
// 		let namespace = self.namespace.as_ref().unwrap();
// 		format!("{}.{}", namespace, self.name)
// 	}
// }

// /// This is just to specify if the `field` in a record is meant to be encoded or decoded
// #[derive(Clone, PartialEq, Debug)]  
// pub enum SchemaVariant {
// 	/// For encoding
// 	Encoded(Type),
// 	/// For Decoding
// 	Decoded(FromAvro)
// }

// impl Encoder for SchemaVariant {
// 	fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, AvroErr> {
// 		if let SchemaVariant::Encoded(ref schm) = *self {
// 			schm.encode(writer)
// 		} else {
// 			unreachable!("encode must be only called on a Encoded variant of any field");
// 		}
// 	}
// }

// /// A field represents the elements of the `fields` attribute of the `Record`
// #[derive(Debug, PartialEq, Clone)]
// pub struct Field {
// 	/// Name of the field in a Record Type
// 	name: String,
// 	/// Optional docs describing the field
// 	doc: Option<String>,
// 	/// The Type of the field
// 	pub ty: SchemaVariant,
// 	/// The default value of this field
// 	default: Option<SchemaVariant>
// }

// impl Field {
// 	/// Create a new field for encoding given its name, schema and an optional doc string.
// 	pub fn new_for_encoding(name: &str, doc: Option<&str>, ty: Type) -> Self {
// 		Field {
// 			name: name.to_string(),
// 			doc: doc.map(|s| s.to_owned()),
// 			ty: SchemaVariant::Encoded(ty),
// 			default: None
// 		}
// 	}
// }

	// /// parses a Record field from a serde_json object
	// /// TODO implement this
	// pub fn from_json(obj: Value) -> Result<Self, ()> {
	// 	if obj.is_object() {
	// 		let f_name = obj.get("name").unwrap().as_str().unwrap();
	// 		Err(())
	// 	} else {
	// 		Err(())
	// 	}
	// }

	// /// Retrieves the name of the field.
	// pub fn get_name(&self) -> &str {
	// 	self.name.as_str()
	// }
// }