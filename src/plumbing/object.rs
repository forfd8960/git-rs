use std::{
    fs::{self, OpenOptions},
    io::Write,
};
use chrono::{self, DateTime, FixedOffset};

use flate2::write::ZlibEncoder;
use flate2::Compression;

// const DateFormat = "Mon Jan 02 15:04:05 2006 -0700"
const DATEFORMAT: &str = "%a %b %d %H:%M:%S %Y %z";

pub enum ObjectType {
    InvalidObject,
    CommitObject,
    TreeObject,
    BlobObject,
    TagObject,
}

pub struct Signature {
    pub name: String,
    pub email: String,
    pub when: DateTime<FixedOffset>,
}

impl Signature {
    pub fn decode(b: &[u8]) -> Self {
        let b_str = String::from_utf8_lossy(b);
        let open = b_str.rfind('<').unwrap_or(0);
        let close_bracket = b_str.rfind('>').unwrap_or(0);

        let name = b_str[..open].trim().to_string();
        let email = b_str[open + 1..close_bracket].to_string();

        let has_time = close_bracket + 2 < b_str.len();
        let when = if has_time {
            let time_str = &b_str[close_bracket + 2..];
            DateTime::parse_from_str(time_str, DATEFORMAT).unwrap()
        } else {
            chrono::Utc::now().with_timezone(&FixedOffset::east(0))
        };

        Signature { name, email, when }
    }

    pub fn encode(&self) -> String {
        format!("{} <{}> {}", self.name, self.email, self.when.format(DATEFORMAT))
    }
}

/*
// DateFormat is the format being used in the original git implementation
const DateFormat = "Mon Jan 02 15:04:05 2006 -0700"

// Signature is used to identify who and when created a commit or tag.
type Signature struct {
	// Name represents a person name. It is an arbitrary string.
	Name string
	// Email is an email, but it cannot be assumed to be well-formed.
	Email string
	// When is the timestamp of the signature.
	When time.Time
}

// Decode decodes a byte slice into a signature
func (s *Signature) Decode(b []byte) {
	open := bytes.LastIndexByte(b, '<')
	closeBracket := bytes.LastIndexByte(b, '>')
	if open == -1 || closeBracket == -1 {
		return
	}

	if closeBracket < open {
		return
	}

	s.Name = string(bytes.Trim(b[:open], " "))
	s.Email = string(b[open+1 : closeBracket])

	hasTime := closeBracket+2 < len(b)
	if hasTime {
		s.decodeTimeAndTimeZone(b[closeBracket+2:])
	}
}

// Encode encodes a Signature into a writer.
func (s *Signature) Encode(w io.Writer) error {
	if _, err := fmt.Fprintf(w, "%s <%s> ", s.Name, s.Email); err != nil {
		return err
	}
	if err := s.encodeTimeAndTimeZone(w); err != nil {
		return err
	}
	return nil
}

var timeZoneLength = 5

func (s *Signature) decodeTimeAndTimeZone(b []byte) {
	space := bytes.IndexByte(b, ' ')
	if space == -1 {
		space = len(b)
	}

	ts, err := strconv.ParseInt(string(b[:space]), 10, 64)
	if err != nil {
		return
	}

	s.When = time.Unix(ts, 0).In(time.UTC)
	tzStart := space + 1
	if tzStart >= len(b) || tzStart+timeZoneLength > len(b) {
		return
	}

	timezone := string(b[tzStart : tzStart+timeZoneLength])
	tzhours, err1 := strconv.ParseInt(timezone[0:3], 10, 64)
	tzmins, err2 := strconv.ParseInt(timezone[3:], 10, 64)
	if err1 != nil || err2 != nil {
		return
	}
	if tzhours < 0 {
		tzmins *= -1
	}

	tz := time.FixedZone("", int(tzhours*60*60+tzmins*60))

	s.When = s.When.In(tz)
}

func (s *Signature) encodeTimeAndTimeZone(w io.Writer) error {
	u := max(s.When.Unix(), 0)
	_, err := fmt.Fprintf(w, "%d %s", u, s.When.Format("-0700"))
	return err
}

func (s *Signature) String() string {
	return fmt.Sprintf("%s <%s>", s.Name, s.Email)
}
*/

pub fn get_object_type(object_type: &str) -> ObjectType {
    match object_type {
        "commit" => ObjectType::CommitObject,
        "tree" => ObjectType::TreeObject,
        "blob" => ObjectType::BlobObject,
        "tag" => ObjectType::TagObject,
        _ => ObjectType::InvalidObject,
    }
}

pub fn object_type_bytes(object_type: &ObjectType) -> &'static [u8] {
    match object_type {
        ObjectType::CommitObject => b"commit",
        ObjectType::TreeObject => b"tree",
        ObjectType::BlobObject => b"blob",
        ObjectType::TagObject => b"tag",
        ObjectType::InvalidObject => b"invalid",
    }
}
pub fn object_type_string(object_type: &ObjectType) -> &'static str {
    match object_type {
        ObjectType::CommitObject => "commit",
        ObjectType::TreeObject => "tree",
        ObjectType::BlobObject => "blob",
        ObjectType::TagObject => "tag",
        ObjectType::InvalidObject => "invalid",
    }
}

pub fn write_blob(content: Vec<u8>, hash_bytes: &[u8]) -> anyhow::Result<String> {
    let content_bytes = content;
    let content_len = content_bytes.len();
    let header = format!("blob {}\0", content_len);
    let mut blob_bs = header.as_bytes().to_vec();

    blob_bs.extend_from_slice(content_bytes.as_slice());

    let hash_str = base16ct::lower::encode_string(hash_bytes);

    let (blob_dir, file_name) = get_blob_path(&hash_str);
    println!("[write_blob] blob dir: {}", blob_dir.clone());
    println!("[write_blob] file_name: {}", file_name.clone());

    fs::create_dir(blob_dir)?;

    let blob = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name.clone())?;
    println!("created file");

    let mut e = ZlibEncoder::new(blob, Compression::default());
    e.write_all(&blob_bs)?;
    e.finish()?;

    Ok(file_name)
}

fn get_blob_path(hash_str: &str) -> (String, String) {
    let dir = ".git/objects/".to_owned() + &hash_str[0..2];
    (dir.clone(), dir + "/" + &hash_str[2..])
}
