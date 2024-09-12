#[cfg(test)]
mod tests {
    use super::super::identifier::{ Identifier, InvalidIdentifierError };

    #[test]
    fn test_valid_with_version() {
        let identifier = Identifier::parse("owner/name:abc123").unwrap();
        assert_eq!(identifier.owner, "owner");
        assert_eq!(identifier.name, "name");
        assert_eq!(identifier.version, Some("abc123".to_string()));
        assert_eq!(identifier.to_string(), "owner:name:abc123");
    }

    #[test]
    fn test_valid_without_version() {
        let identifier = Identifier::parse("black-forest-labs/flux-schnell").unwrap();
        assert_eq!(identifier.owner, "owner");
        assert_eq!(identifier.name, "name");
        assert_eq!(identifier.version, None);
        assert_eq!(identifier.to_string(), "owner:name");
    }

    #[test]
    fn test_invalid() {
        assert!(matches!(Identifier::parse("invalid"), Err(InvalidIdentifierError)));
    }

    #[test]
    fn test_empty() {
        assert!(matches!(Identifier::parse("/"), Err(InvalidIdentifierError)));
    }

    #[test]
    fn test_blank() {
        assert!(matches!(Identifier::parse(""), Err(InvalidIdentifierError)));
    }
}
