use std::{collections::HashMap, str::FromStr};

use lazy_regex::regex_captures;
use simple_error::SimpleError;

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
            let (_, dir_name) = regex_captures!(r"^dir (\w+)$", s).ok_or(SimpleError::new(
                "Couldn't extract directory name from dir output",
            ))?;
            return Ok(TerminalOutput::DirectoryEntry(dir_name.to_string()));
        } else {
            // Try to match the file entry
            let (_, file_size_str, file_name) = regex_captures!(r"^(\d+) ([\w\.]+)$", s)
                .ok_or(SimpleError::new("Couldn't parse file entry"))?;
            let file_size: u64 = file_size_str.parse().unwrap(); // This shouldn't panic if the regex matched
            return Ok(TerminalOutput::FileEntry(file_size, file_name.to_string()));
        }
    }
}

fn compute_full_path(active_dirs: &Vec<String>, dirname: &str) -> String {
    if active_dirs.len() == 1 {
        return format!("/{}", dirname).to_string();
    }

    let sep = "/";

    let mut path = active_dirs.last().unwrap().clone();
    path.push_str(sep);
    path.push_str(dirname);
    path
}

struct ParseHelper {
    active_dirs: Vec<String>,
    dir_sizes: HashMap<String, u64>,
}

impl ParseHelper {
    fn new() -> Self {
        ParseHelper {
            active_dirs: vec!["/".to_string()],
            dir_sizes: HashMap::new(),
        }
    }

    fn process_output(&mut self, output: TerminalOutput) {
        use TerminalOutput::*;
        match output {
            CommandCDRoot => self.reset_active_dirs(),
            CommandCDUp => self.move_up(),
            CommandCDSSub(dirname) => self.move_down(&dirname),
            CommandLS => {}
            DirectoryEntry(_) => {}
            FileEntry(size, _) => self.add_file_size_to_active_dirs(size) 
        }
    }

    fn add_file_size_to_active_dirs(&mut self, size: u64) {
        for dir in &self.active_dirs {
            let entry: &mut u64 = self.dir_sizes.entry(dir.clone()).or_default();
            *entry += size;
        }
    }

    fn reset_active_dirs(&mut self) {
        self.active_dirs.drain(1..);
    }

    fn move_up(&mut self) {
        if self.active_dirs.len() > 1 {
            self.active_dirs.pop();
        }
    }

    fn move_down(&mut self, dirname: &str) {
        let dirname = compute_full_path(&self.active_dirs, &dirname);
        self.active_dirs.push(dirname);
    }
}

pub fn parse_terminal_output_for_dir_sizes(output: &str) -> HashMap<String, u64> {
    let mut helper = ParseHelper::new();

    for line in output.lines() {
        let output: TerminalOutput = line.parse().unwrap();

        helper.process_output(output);
    }

    helper.dir_sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_of_terminal_output() {
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
        assert_eq!(
            output,
            TerminalOutput::DirectoryEntry("foobifoo".to_string())
        );

        let filenames = vec!["foo", "foo.txt", "foo_bar.txt.bak"];
        for filename in filenames {
            let output: TerminalOutput = format!("1234 {}", filename).parse().unwrap();
            let expected = TerminalOutput::FileEntry(1234, filename.to_string());
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn test_processing() {
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

        let dir_sizes = parse_terminal_output_for_dir_sizes(TEST_INPUT);

        assert_eq!(dir_sizes.get("/a/e"), Some(&584));
        assert_eq!(dir_sizes.get("/a"), Some(&94853));
        assert_eq!(dir_sizes.get("/d"), Some(&24933642));
        assert_eq!(dir_sizes.get("/"), Some(&48381165));
    }

    #[test]
    fn test_full_path_comp() {
        let active_dirs = vec!["/".to_string(), "/b".to_string()];
        let dirname = "c";

        let result = compute_full_path(&active_dirs, &dirname);
        assert_eq!(result, "/b/c".to_string());
    }
}
