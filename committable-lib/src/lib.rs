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

trait Rule {
    fn check(&self, commit: &Commit) -> Result<(), ()>;
}

struct NonEmptyHeader {}
impl Rule for NonEmptyHeader {
    fn check(&self, commit: &Commit) -> Result<(), ()> {
        if commit.get_header().is_empty() {
            Err(())
        } else {
            Ok(())
        }
    }
}

struct NonEmptyBody {}
impl Rule for NonEmptyBody {
    fn check(&self, commit: &Commit) -> Result<(), ()> {
        let content = commit.commit_string;
        let Some((_, rest)) = content.split_once('\n') else {
            return Ok(());
        };

        if !rest.is_empty() && commit.get_body().is_none() {
            Err(())
        } else {
            Ok(())
        }
    }
}

struct SingleEmptyLineBeforeBody {}

impl Rule for SingleEmptyLineBeforeBody {
    fn check(&self, commit: &Commit) -> Result<(), ()> {
        let content = commit.commit_string;
        let Some((_, rest)) = content.split_once('\n') else {
            return Ok(());
        };

        if !rest.is_empty() && !rest.starts_with('\n') && !rest.starts_with("\r\n") {
            return Err(());
        }

        let (_, rest) = rest.split_once('\n').unwrap_or(("", rest));

        if rest.starts_with('\n') || rest.starts_with("\r\n") {
            return Err(());
        }

        Ok(())
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

    #[test]
    fn nonempty_header() {
        let c = NonEmptyHeader {};
        assert_eq!(c.check(&Commit::new("")), Err(()));
        assert_eq!(c.check(&Commit::new("\nTest")), Err(()));
        assert_eq!(c.check(&Commit::new("\n\nTest")), Err(()));
        assert_eq!(c.check(&Commit::new("YEP\n\nTest")), Ok(()));
        assert_eq!(c.check(&Commit::new("YEP")), Ok(()));
    }

    #[test]
    fn nonempty_body() {
        let c = NonEmptyBody {};
        assert_eq!(c.check(&Commit::new("H")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n\nTest")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n\n\nTest")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\nYEP\n\nTest")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\nYEP")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n\n")), Err(()));
    }

    #[test]
    fn single_empty_line_before_body() {
        let c = SingleEmptyLineBeforeBody {};
        assert_eq!(c.check(&Commit::new("H")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n\nTest")), Ok(()));
        assert_eq!(c.check(&Commit::new("H\n\n\nTest")), Err(()));
        assert_eq!(c.check(&Commit::new("H\nYEP\n\nTest")), Err(()));
        assert_eq!(c.check(&Commit::new("H\nYEP")), Err(()));
        assert_eq!(c.check(&Commit::new("H\n\nNice!\n\n")), Ok(()));
    }
}
