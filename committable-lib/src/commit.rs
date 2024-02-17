pub struct Commit<'a> {
    commit_string: &'a str,
}

impl<'a> Commit<'a> {
    pub fn new(commit_string: &'a str) -> Self {
        Commit { commit_string }
    }

    pub fn get_commit_string(&self) -> &'a str {
        self.commit_string
    }

    pub fn get_header(&self) -> &'a str {
        self.commit_string.lines().next().unwrap_or("")
    }

    pub fn get_body(&self) -> Option<&'a str> {
        let Some((_, rest)) = self.commit_string.split_once('\n') else {
            return None;
        };

        let mut body = rest;
        while body.starts_with('\n') || body.starts_with("\r\n") {
            let Some((_, rest)) = body.split_once('\n') else {
                return None;
            };
            body = rest;
        }

        if body.is_empty() {
            None
        } else {
            Some(body)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_header() {
        let c = Commit::new("Hello world");
        assert_eq!(c.get_commit_string(), "Hello world");
        assert_eq!(c.get_header(), "Hello world");
        assert_eq!(c.get_body(), None);
    }

    #[test]
    fn empty_body() {
        let c = Commit::new("Hello world\n\n");
        assert_eq!(c.get_commit_string(), "Hello world\n\n");
        assert_eq!(c.get_header(), "Hello world");
        assert_eq!(c.get_body(), None);
    }

    #[test]
    fn some_body() {
        let c = Commit::new("Hello world\nSome Body\n");
        assert_eq!(c.get_commit_string(), "Hello world\nSome Body\n");
        assert_eq!(c.get_header(), "Hello world");
        assert_eq!(c.get_body(), Some("Some Body\n"));
    }

    #[test]
    fn more_body() {
        let c = Commit::new("Hello world\n\nSome Body");
        assert_eq!(c.get_commit_string(), "Hello world\n\nSome Body");
        assert_eq!(c.get_header(), "Hello world");
        assert_eq!(c.get_body(), Some("Some Body"));
    }
}
