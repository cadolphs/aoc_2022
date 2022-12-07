use std::str::FromStr;

use lazy_regex::regex_captures;
use simple_error::SimpleError;

pub fn run_day_07(input: String) {
    todo!()
}

#[derive(Debug, PartialEq)]
enum TerminalOutput {
    CommandCDRoot,
    CommandCDUp,
    CommandCDSSub(String),
    CommandLS,
    DirectoryEntry(String),
    FileEntry(u64, String),
}

impl FromStr for TerminalOutput {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(SimpleError::new(
                "Empty string not a valid terminal output to parse",
            ));
        }

        if s.starts_with('$') {
            if s == "$ ls" {
                return Ok(TerminalOutput::CommandLS);
            } else if s == "$ cd /" {
                return Ok(TerminalOutput::CommandCDRoot);
            } else if s == "$ cd .." {
                return Ok(TerminalOutput::CommandCDUp);
            } else {
                let (_, name) = regex_captures!(r"^\$ cd (\w+)$", s).ok_or(SimpleError::new(
                    "Couldn't extract directory name from cd command",
                ))?;
                return Ok(TerminalOutput::CommandCDSSub(name.to_string()));
            }
        } else if s.starts_with("dir") {
            let (_, dir_name) = regex_captures!(r"^dir (\w+)$", s).ok_or(SimpleError::new("Couldn't extract directory name from dir output"))?;
            return Ok(TerminalOutput::DirectoryEntry(dir_name.to_string()));
        } else {
            // Try to match the file entry
            let (_, file_size_str, file_name) = regex_captures!(r"^(\d+) ([\w\.]+)$", s).ok_or(SimpleError::new("Couldn't parse file entry"))?;
            let file_size: u64 = file_size_str.parse().unwrap(); // This shouldn't panic if the regex matched
            return Ok(TerminalOutput::FileEntry(file_size, file_name.to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc!(
        "
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k"
    );

    #[test]
    fn input_hacking() {
        // Just trying out some things about parsing
        let output: TerminalOutput = "$ cd /".parse().unwrap();
        assert_eq!(output, TerminalOutput::CommandCDRoot);

        let output: TerminalOutput = "$ cd ..".parse().unwrap();
        assert_eq!(output, TerminalOutput::CommandCDUp);

        let output: TerminalOutput = "$ cd foo_bar".parse().unwrap();
        assert_eq!(output, TerminalOutput::CommandCDSSub("foo_bar".to_string()));

        let output: TerminalOutput = "$ ls".parse().unwrap();
        assert_eq!(output, TerminalOutput::CommandLS);

        let output: TerminalOutput = "dir foobifoo".parse().unwrap();
        assert_eq!(output, TerminalOutput::DirectoryEntry("foobifoo".to_string()));

        let filenames = vec!["foo", "foo.txt", "foo_bar.txt.bak"];
        for filename in filenames {
            let output: TerminalOutput = format!("1234 {}", filename).parse().unwrap();
            let expected = TerminalOutput::FileEntry(1234, filename.to_string());
            assert_eq!(output, expected);
        }
    }
}
