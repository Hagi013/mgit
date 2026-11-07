fn to_hex(bytes: &[u8]) -> String {
  bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
}

fn from_hex(s: &str) -> Option<Vec<u8>> {
  if s.len() % 2 != 0 { return None; }
  (0..s.len())
    .step_by(2)
    .map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok())
    .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId([u8; 20]);

impl ObjectId {
  pub const LEN: usize = 20;
  pub fn from_raw(bytes: [u8; 20]) -> Self {
    Self(bytes)
  }
  pub fn as_bytes(&self) -> &[u8; 20] {
    &self.0
  }
  pub fn to_hex(&self) -> String {
    to_hex(&self.0)
  }
  /// ("ce", "013625...") の組。Step 9でファイルパスに使う
  pub fn loose_path(&self) -> (String, String) {
    (to_hex(&self.0[..1]), to_hex(&self.0[1..]))
  }
}

impl TryFrom<&[u8]> for ObjectId {
  type Error = crate::error::Error;
  fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
    if b.len() != Self::LEN {
      return Err(crate::error::Error::Invalid(format!("Invalid ObjectId Length: {}", b.len())))
    }
    Ok(Self::from_raw(b.try_into().unwrap()))
  }
}
impl std::str::FromStr for ObjectId {
  type Err = crate::error::Error;
  fn from_str(b: &str) -> Result<Self, Self::Err> {
    if b.len() != 40 || !b.bytes().all(|c| c.is_ascii_hexdigit()) {
      return Err(crate::error::Error::Invalid(format!("Invalid ObjectId Length or Invalid ObjectId Format: {}", b.len())))
    }
    let hex = from_hex(b)
      .ok_or_else(|| crate::error::Error::Invalid(format!("Invalid ObjectId Hex: {}", b)))?;
    Ok(Self::try_from(hex.as_slice())?)
  }
}
impl std::fmt::Display for ObjectId {
  /* 40桁hex */
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.to_hex(), f)
  }
}
impl std::fmt::Debug for ObjectId {
  /* 先頭7桁 */
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.to_hex()[..7], f)
  }
}

#[cfg(test)]
mod test {
  use sha1::{Digest, Sha1};
  use super::*;
  #[test]
  fn test_to_hex() {
    let bytes = [0x01, 0x02, 0x03, 0x04];
    let hex = to_hex(&bytes);
    assert_eq!(hex, "01020304");
  }

  #[test]
  fn test_loose_path() {
    let string = "blob 6\0hello\n";
    let (dir, file) = Sha1::new()
      .chain_update(string.as_bytes())
      .finalize()
      .as_slice()
      .try_into()
      .map(ObjectId::from_raw)
      .map(|id| {
        id.loose_path()
      }).unwrap();
    assert_eq!(dir, "ce");
    assert_eq!(file, "013625030ba8dba906f756967f9e9ca394464a");
  }

  const HELLO: &str = "ce013625030ba8dba906f756967f9e9ca394464a";

  #[test]
  fn test_parse_roundtrip() {
    let id = HELLO.parse::<ObjectId>().unwrap();
    assert_eq!(id.to_string(), HELLO);
    assert_eq!(id.to_hex(), HELLO);
  }

  #[test]
  fn test_parse_rejects_wrong_length() {
    assert!(HELLO[..39].parse::<ObjectId>().is_err());
    assert!(format!("{HELLO}0").parse::<ObjectId>().is_err());
    assert!("".parse::<ObjectId>().is_err());
  }

  #[test]
  fn test_parse_rejects_non_hex() {
    // 40文字だが hex でない文字を含む
    assert!(format!("g{}", &HELLO[1..]).parse::<ObjectId>().is_err());
    assert!(format!("+{}", &HELLO[1..]).parse::<ObjectId>().is_err());
    assert!(format!(" {}", &HELLO[1..]).parse::<ObjectId>().is_err());
    // バイト長は40だがマルチバイト文字を含む
    assert!(format!("あ{}", &HELLO[3..]).parse::<ObjectId>().is_err());
  }

  #[test]
  fn test_debug_is_short() {
    let id = HELLO.parse::<ObjectId>().unwrap();
    assert_eq!(format!("{id:?}"), "ce01362");
  }

  #[test]
  fn test_try_from_bytes() {
    let id = HELLO.parse::<ObjectId>().unwrap();
    assert_eq!(ObjectId::try_from(&id.as_bytes()[..]).unwrap(), id);
    assert!(ObjectId::try_from(&id.as_bytes()[..19]).is_err());
    assert!(ObjectId::try_from(&[0u8; 21][..]).is_err());
  }
}
