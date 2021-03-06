#[cfg(test)]
mod tests {
    use crate::scfmt;
    use crate::scfmt::ScfmtErr;
    use std::fs;

    //> basic tests
        #[test]
        fn format_str() {
            let to_format = fs::read_to_string("./test_resources/1_test.rs").unwrap();
            let answer = fs::read_to_string("./test_resources/1_answer.rs").unwrap();
            let formatted = scfmt::format_str(&to_format, "rs").unwrap();
            assert_eq!(answer, formatted);
        }

        #[test]
        fn add_brackets() {
            let to_format = fs::read_to_string("./test_resources/2_test.rs").unwrap();
            let answer = fs::read_to_string("./test_resources/2_answer.rs").unwrap();
            let formatted = scfmt::add_brackets(&to_format, "rs").unwrap();
            assert_eq!(answer, formatted);
        }

        #[test]
        fn remove_brackets() {
            let to_format = fs::read_to_string("./test_resources/3_test.rs").unwrap();
            let answer = fs::read_to_string("./test_resources/3_answer.rs").unwrap();
            let formatted = scfmt::remove_brackets(&to_format, "rs").unwrap();
            assert_eq!(answer, formatted);
        }

        #[test]
        fn no_change_without_brackets() {
            let before_formatting = fs::read_to_string("./test_resources/4_test.rs").unwrap();
            let formatted = scfmt::format_str(&before_formatting, "rs").unwrap();
            assert_eq!(formatted, before_formatting);
        }

    //<> Brackets not closed properly
        #[test]
        fn no_head_for_closing() {
            let before_formatting = fs::read_to_string("./test_resources/5_test.rs").unwrap();
            let formatted = scfmt::format_str(&before_formatting, "rs");
            assert_eq!(formatted, Err(ScfmtErr::CommentClosedNothing(46)));
        }

        #[test]
        fn no_head_for_middle() {
            let before_formatting = fs::read_to_string("./test_resources/6_test.rs").unwrap();
            let formatted = scfmt::format_str(&before_formatting, "rs");
            assert_eq!(formatted, Err(ScfmtErr::CommentClosedNothing(21)));
        }

        #[test]
        fn head_never_closed() {
            let before_formatting = fs::read_to_string("./test_resources/7_test.rs").unwrap();
            let formatted = scfmt::format_str(&before_formatting, "rs");
            assert_eq!(formatted, Err(ScfmtErr::CommentNeverClosed(1)));
        }

    //<> ending empty lines are preserved
        #[test]
        fn format_preserves_ending_empty_lines() {
            //> empty input
                let formatted = scfmt::format_str("", "rs").unwrap();
                assert_eq!(formatted, "");
            //<> 0 empty ending lines
                let formatted = scfmt::format_str("//>\n//<", "rs").unwrap();
                assert_eq!(formatted, "//>\n//<");
            //<> 1 empty ending lines
                let formatted = scfmt::format_str("//>\n//<\n", "rs").unwrap();
                assert_eq!(formatted, "//>\n//<\n");
            //<> 2 empty ending lines
                let formatted = scfmt::format_str("//>\n//<\n\n", "rs").unwrap();
                assert_eq!(formatted, "//>\n//<\n\n");
            //<> 3 empty ending lines
                let formatted = scfmt::format_str("//>\n//<\n\n\n", "rs").unwrap();
                assert_eq!(formatted, "//>\n//<\n\n\n");
            //<> 3 empty ending lines with space on last line
                let formatted = scfmt::format_str("//>\n//<\n\n\n ", "rs").unwrap();
                assert_eq!(formatted, "//>\n//<\n\n\n");
            //<
        }

        #[test]
        fn remove_brackets_preserves_ending_empty_lines() {
            //> empty input
                let formatted = scfmt::remove_brackets("", "rs").unwrap();
                assert_eq!(formatted, "");
            //<> 0 empty ending lines
                let formatted = scfmt::remove_brackets("//>\n    let a = 0;\n//<", "rs").unwrap();
                assert_eq!(formatted, "//\n    let a = 0;");
            //<> 1 empty ending lines
                let formatted = scfmt::remove_brackets("//>\n    let a = 0;\n//<\n", "rs").unwrap();
                assert_eq!(formatted, "//\n    let a = 0;\n");
            //<> 2 empty ending lines
                let formatted = scfmt::remove_brackets("//>\n    let a = 0;\n//<\n\n", "rs").unwrap();
                assert_eq!(formatted, "//\n    let a = 0;\n\n");
            //<> 3 empty ending lines
                let formatted = scfmt::remove_brackets("//>\n    let a = 0;\n//<\n\n\n", "rs").unwrap();
                assert_eq!(formatted, "//\n    let a = 0;\n\n\n");
            //<> 3 empty ending lines with space on last line
                let formatted = scfmt::remove_brackets("//>\n    let a = 0;\n//<\n\n\n ", "rs").unwrap();
                assert_eq!(formatted, "//\n    let a = 0;\n\n\n");
            //<
        }

        #[test]
        fn add_brackets_preserves_ending_empty_lines() {
            //> empty input
                let formatted = scfmt::add_brackets("", "rs").unwrap();
                assert_eq!(formatted, "");
            //<> 0 empty ending lines
                let formatted = scfmt::add_brackets("//\n    let a = 0;", "rs").unwrap();
                assert_eq!(formatted, "//>\n    let a = 0;\n//<");
            //<> 1 empty ending lines
                let formatted = scfmt::add_brackets("//\n    let a = 0;\n", "rs").unwrap();
                assert_eq!(formatted, "//>\n    let a = 0;\n//<\n");
            //<> 2 empty ending lines
                let formatted = scfmt::add_brackets("//\n    let a = 0;\n\n", "rs").unwrap();
                assert_eq!(formatted, "//>\n    let a = 0;\n//<\n\n");
            //<> 3 empty ending lines
                let formatted = scfmt::add_brackets("//\n    let a = 0;\n\n\n", "rs").unwrap();
                assert_eq!(formatted, "//>\n    let a = 0;\n//<\n\n\n");
            //<> 3 empty ending lines with space at end
                let formatted = scfmt::add_brackets("//\n    let a = 0;\n\n\n ", "rs").unwrap();
                assert_eq!(formatted, "//>\n    let a = 0;\n//<\n\n\n");
            //<
        }

        #[test]
        fn null_brackets_preserves_ending_empty_lines() {
            //> empty input
                let formatted = scfmt::null_existing_brackets("", "rs").unwrap();
                assert_eq!(formatted, "");
            //<> 0 empty ending lines
                let formatted = scfmt::null_existing_brackets("//>\n//<", "rs").unwrap();
                assert_eq!(formatted, "//_>\n//_<");
            //<> 1 empty ending lines
                let formatted = scfmt::null_existing_brackets("//>\n//<\n", "rs").unwrap();
                assert_eq!(formatted, "//_>\n//_<\n");
            //<> 2 empty ending lines
                let formatted = scfmt::null_existing_brackets("//>\n//<\n\n", "rs").unwrap();
                assert_eq!(formatted, "//_>\n//_<\n\n");
            //<> 3 empty ending lines
                let formatted = scfmt::null_existing_brackets("//>\n//<\n\n\n", "rs").unwrap();
                assert_eq!(formatted, "//_>\n//_<\n\n\n");
            //<> 3 empty ending lines with space at end
                let formatted = scfmt::null_existing_brackets("//>\n//<\n\n\n ", "rs").unwrap();
                assert_eq!(formatted, "//_>\n//_<\n\n\n");
            //<
        }

    //<> tabs
        #[test]
        fn format_str_tabs() {
            let to_format = fs::read_to_string("./test_resources/9_test.rs").unwrap();
            let answer = fs::read_to_string("./test_resources/9_answer.rs").unwrap();
            let formatted = scfmt::format_str(&to_format, "rs").unwrap();
            assert_eq!(answer, formatted);
        }
    //<> tab_spaces of 2 (two spaces per indent)
        #[test]
        fn format_str_tab_spaces_of_2() {
            let to_format = fs::read_to_string("./test_resources/10_test.rs").unwrap();
            let answer = fs::read_to_string("./test_resources/10_answer.rs").unwrap();
            let formatted = scfmt::format_str(&to_format, "rs").unwrap();
            assert_eq!(answer, formatted);
        }
    //<> comment contents on closing brackets are preserved
        #[test]
        fn preserve_closing_comment_content() {
            let formatted = scfmt::format_str("//>\n//< test", "rs").unwrap();
            assert_eq!(formatted, "//>\n//<\n// test");
        }

        #[test]
        fn preserve_closing_comment_content_and_spacing() {
            let formatted = scfmt::format_str("//>\n// < test", "rs").unwrap();
            assert_eq!(formatted, "//>\n// <\n// test");
        }
    //<
    #[test]
    fn nullify_brackets() {
        let formatted =
            scfmt::null_existing_brackets("//>\n    //>\n//\n    //<\n//<", "rs").unwrap();
        assert_eq!(formatted, "//_>\n    //_>\n//\n    //_<\n//_<");
    }

    #[test]
    fn incompatible_file_type() {
        let result = scfmt::format_str("", "");
        assert_eq!(result, Err(ScfmtErr::IncompatibleFileType));

        let result = scfmt::add_brackets("", "");
        assert_eq!(result, Err(ScfmtErr::IncompatibleFileType));

        let result = scfmt::remove_brackets("", "");
        assert_eq!(result, Err(ScfmtErr::IncompatibleFileType));

        let result = scfmt::null_existing_brackets("", "");
        assert_eq!(result, Err(ScfmtErr::IncompatibleFileType));
    }

    #[test]
    fn determine_whitespace_type_gets_best_result() {
        let to_format = fs::read_to_string("./test_resources/11_test.rs").unwrap();
        let formatted = scfmt::format_str(&to_format, "rs").unwrap();
        let answer = fs::read_to_string("./test_resources/11_answer.rs").unwrap();
        assert_eq!(answer, formatted);
    }
}

/// Contains functions for formatting strucuted comments in files and strings
pub mod scfmt {

    use glob::{glob, GlobError};
    use phf::phf_map;
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    static EXTENSION_TO_COMMENT_STARTER_MAP: phf::Map<&'static str, &'static str> = phf_map! {

        //> ada
            "adb" => "--",
            "ads" => "--",
        //<
        // Assembly
        "asm" => ";",
        // AL
        "al" => "//",
        "bib" => "%",
        "brs" => "'",
        // C
        "c" => "//",
        "cfc" => "//",
        // Clojure
        "clj" => ";",
        // Apex
        "cls" => "//",
        "cpp" => "//",
        //> C#
            "cs" => "//",
            "csx" => "//",
        //<
        "d" => "//",
        // Dart
        "dart" => "//",
        "do" => "*",
        "ex" => "#",
        "elm" => "--",
        "gd" => "#",
        "gen" => "\\",
        // Go
        "go" => "//",
        "graphql" => "#",
        "groovy" => "//",
        // C header files
        "h" => "//",
        //> Haskell
            "hs" => "--",
            "lhs" => "--",
        //<
        // Java
        "java" => "//",
        //> JavaScript
            "js" => "//",
            "cjs" => "//",
            "mjs" => "//",
        //<
        "jsonc" => "//",
        "lisp" => ";;",
        "lua" => "--",
        // MATLAB
        "m" => "%",
        "nim" => "#",
        // Pascal
        "pas" => "//",
        // PHP
        "php" => "//",
        "pig" => "--",
        "plsql" => "--",
        "pp" => "//",
        "ps1" => "#",
        "pu" => "'",
        "q" => "--",
        "rkt" => ";",
        // Rust
        "rs" => "//",
        "sas" => "*",
        "sass" => "//",
        "scss" => "//",
        "shader" => "//",
        // Bash
        "sh" => "#",
        // Solidity
        "sol" => "//",
        "styl" => "//",
        "svelte" => "//",
        "tcl" => "#",
        "toml" => "#",
        //> TypeScript
            "ts" => "//",
            "tsx" => "//",
        //<
        "vala" => "//",
        "v" => "//",
        "vhdl" => "--",
        "vue" => "//",
        "yaml" => "#",
    };

    /// Enum used to represent scfmt errors
    #[derive(PartialEq, Debug)]
    pub enum ScfmtErr {
        IncompatibleFileType,
        CommentClosedNothing(usize),
        CommentNeverClosed(usize),
        CantConvertOsString,
        CantReadFileAsString,
        CantCreatFile,
        CantWriteToFile,
    }

    fn determine_whitespace_type(str: &str) -> (char, usize) {
        //> if no whitespace is found, assume format is 4 spaces
            let mut chr = ' ';
            let mut num = 4;
        //<

        let mut tab_count = 0;
        let mut space_count = 0;
        let mut tab_spaces_count_map: HashMap<usize, usize> = HashMap::new();
        let mut last_depth;
        let mut last_diff = 0;
        let mut cur_depth = 0;

        for line in str.lines() {
            // if line is not empty
            if let Some(first_char) = line.chars().next() {
                //> get dif between this line and last line
                    last_depth = cur_depth;
                    cur_depth =
                        if let Some((local_depth, _)) = count_and_remove_begining_whitespace(line) {
                            local_depth
                        } else {
                            0
                        };

                    let diff = (last_depth as isize - cur_depth as isize).abs() as usize;
                //<

                match first_char {
                    ' ' => space_count += 1,
                    '\t' => tab_count += 1,
                    _ => {}
                }

                // store diff
                if diff != 0 {
                    // count the current diff

                    match tab_spaces_count_map.get(&diff) {
                        Some(x) => {
                            let current_map_value = x.clone();
                            tab_spaces_count_map.insert(diff, current_map_value + 1)
                        }
                        None => tab_spaces_count_map.insert(diff, 1),
                    };

                    last_diff = diff;
                } else {
                    // if there was no change in diff, count the last_diff

                    if last_diff != 0 {
                        match tab_spaces_count_map.get(&last_diff) {
                            Some(x) => {
                                let current_map_value = x.clone();
                                tab_spaces_count_map.insert(last_diff, current_map_value + 1)
                            }
                            None => tab_spaces_count_map.insert(last_diff, 1),
                        };
                    }
                }
            }
        }

        //> determine most often occuring diff
            let mut highest_count = 0;
            let mut diff_with_highest_count = 0;
            for (diff_size, diff_count) in &tab_spaces_count_map {
                if diff_count > &highest_count {
                    highest_count = *diff_count;
                    diff_with_highest_count = *diff_size;
                }
            }

        //<> determine num and char
            if diff_with_highest_count != 0 {
                num = diff_with_highest_count;

                if tab_count > space_count {
                    chr = '\t'
                }
            }
        //<

        (chr, num)
    }

    fn add_whitespace(line: &str, depth: usize, whitespace_char: char) -> String {
        let mut value = String::from("");

        for _i in 0..depth {
            value.push(whitespace_char);
        }

        value + line
    }

    fn set_whitespace(str: &str, depth: usize, whitespace_char: char) -> String {
        let str_no_whitespace = match count_and_remove_begining_whitespace(str) {
            Some(x) => x.1,
            None => "".to_owned(),
        };

        //> generate whitespace
            let mut whitespace = String::from("");
            for _i in 0..depth {
                whitespace.push(whitespace_char);
            }
        //<

        whitespace + &str_no_whitespace
    }

    /// Returns a list of all files in a directory and it's subdirectories
    pub fn get_files_in_dir(path: &str, filetype: &str) -> Result<Vec<PathBuf>, GlobError> {
        //> get list of all files and dirs in path, using glob
            let mut paths = Vec::new();

            let mut potential_slash = "";
            if PathBuf::from(path).is_dir() && !path.ends_with('/') {
                potential_slash = "/";
            }

            let search_params = String::from(path) + potential_slash + "**/*" + filetype;

            for entry in glob(&search_params).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        paths.push(path);
                    }
                    Err(e) => return Err(e),
                }
            }

        //<> filter out directories
            let paths = paths.into_iter().filter(|e| e.is_file());

        //<> filter out non unicode files
            let paths: Vec<PathBuf> = paths
                .into_iter()
                .filter(|e| fs::read_to_string(e).is_ok())
                .collect();
        //<
        Ok(paths)
    }

    fn ensure_previous_lines_have_correct_indentation(
        formatted_lines: &mut [String],
        comment_tracker: &mut [CommentDetail],
        tab_spaces: usize,
        whitespace_char: char,
    ) {
        //> determine how much whitespace should be added
            let mut lowest_depth = comment_tracker[comment_tracker.len() - 1].depth + tab_spaces;
            let line_of_last_unclosed_comment = comment_tracker[comment_tracker.len() - 1].line;
            for i in line_of_last_unclosed_comment + 1..formatted_lines.len() {
                let whitespaces_option = count_and_remove_begining_whitespace(&formatted_lines[i]);
                match whitespaces_option {
                    Some(spaces_tuple) => {
                        if spaces_tuple.0 < lowest_depth {
                            lowest_depth = spaces_tuple.0;
                        }
                    }
                    None => continue,
                }
            }
        //<> add any needed whitespace
            if lowest_depth < comment_tracker[comment_tracker.len() - 1].depth + tab_spaces {
                let depth_difference =
                    comment_tracker[comment_tracker.len() - 1].depth + tab_spaces - lowest_depth;
                if depth_difference > 0 {
                    for i in line_of_last_unclosed_comment + 1..formatted_lines.len() {
                        match count_and_remove_begining_whitespace(&formatted_lines[i]) {
                            Some(_) => {
                                formatted_lines[i] = add_whitespace(
                                    &formatted_lines[i],
                                    depth_difference,
                                    whitespace_char,
                                )
                            }
                            None => formatted_lines[i] = "\n".to_owned(),
                        }
                    }
                }
            }
        //<
    }

    fn chop_off_beginning_spaces(line: &str) -> (Option<usize>, &str) {
        let mut line_no_leading_spaces = "";
        let mut leading_spaces: Option<usize> = None;
        for (i, char) in line.chars().enumerate() {
            if char as u32 > 32 {
                line_no_leading_spaces = &line[i..];
                leading_spaces = Some(i);
                break;
            }
        }

        (leading_spaces, line_no_leading_spaces)
    }

    fn remove_comment_notation_if_it_exists(
        line: &str,
        comment_starter: &str,
    ) -> (bool, bool, String) {
        let mut line_no_comment_starter = line;
        let comment_starter_with_space = comment_starter.to_owned() + " ";
        let mut is_a_comment = false;
        let mut space_after_comment_starter = false;
        if line_no_comment_starter.starts_with(&comment_starter_with_space) {
            is_a_comment = true;
            space_after_comment_starter = true;
            line_no_comment_starter = &line_no_comment_starter[comment_starter.len() + 1..];
        } else if line_no_comment_starter.starts_with(comment_starter) {
            is_a_comment = true;
            space_after_comment_starter = false;
            line_no_comment_starter = &line_no_comment_starter[comment_starter.len()..];
        }

        (
            is_a_comment,
            space_after_comment_starter,
            line_no_comment_starter.to_owned(),
        )
    }

    /// Ensures lines inside bracketed structured comments are indented
    ///
    /// # Arguments
    ///
    /// * `str` - A string slice to be formatted
    /// * `filetype` - A string slice of the file extension representing what language arg `str` is.
    ///
    /// # Examples
    ///
    /// ```
    /// use scfmt::scfmt::format_str;
    ///
    /// let to_format = "
    /// //>
    /// //this comment and the line below will be indented after formatting
    /// let a = 0;
    /// //<";
    /// 
    /// let formatted = "
    /// //>
    ///     //this comment and the line below will be indented after formatting
    ///     let a = 0;
    /// //<";
    ///
    /// assert_eq!(formatted, format_str(&to_format, "rs").unwrap());
    /// ```
    pub fn format_str(str: &str, filetype: &str) -> Result<String, ScfmtErr> {
        // determine if file compatible
        let comment_starter = match EXTENSION_TO_COMMENT_STARTER_MAP.get(filetype) {
            Some(x) => *x,
            None => return Err(ScfmtErr::IncompatibleFileType),
        };

        let mut formatted_file = String::from("");
        let mut formatted_lines: Vec<String> = Vec::new();
        let (whitespace_char, tab_spaces) = determine_whitespace_type(str);
        let mut comment_tracker: Vec<CommentDetail> = Vec::new();

        for (i, line) in str.lines().enumerate() {
            // chop off begining spaces
            let (leading_spaces, line_no_leading_spaces) = chop_off_beginning_spaces(line);

            // remove comment notation if it exists
            let (is_a_comment, space_after_comment_starter, line_no_comment_starter) =
                remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

            //> apply whitespace depth
                if is_a_comment & line_no_comment_starter.starts_with('>') {
                    formatted_lines.push(line.to_string() + "\n");

                    //> add comment to comment tracker
                        let comment = CommentDetail {
                            line: i,
                            depth: leading_spaces.unwrap(),
                        };
                        comment_tracker.push(comment);
                    //<
                } else if is_a_comment & line_no_comment_starter.starts_with("<>") {
                    if comment_tracker.is_empty() {
                        return Err(ScfmtErr::CommentClosedNothing(i + 1));
                    }

                    ensure_previous_lines_have_correct_indentation(
                        &mut formatted_lines,
                        &mut comment_tracker,
                        tab_spaces,
                        whitespace_char,
                    );

                    formatted_lines.push(
                        set_whitespace(
                            line,
                            comment_tracker[comment_tracker.len() - 1].depth,
                            whitespace_char,
                        ) + "\n",
                    );

                    //> remove and add comment to comment tracker
                        let comment = CommentDetail {
                            line: i,
                            depth: comment_tracker[comment_tracker.len() - 1].depth,
                        };
                        comment_tracker.pop();
                        comment_tracker.push(comment);
                    //<
                } else if is_a_comment & line_no_comment_starter.starts_with('<') {
                    if comment_tracker.is_empty() {
                        return Err(ScfmtErr::CommentClosedNothing(i + 1));
                    }

                    ensure_previous_lines_have_correct_indentation(
                        &mut formatted_lines,
                        &mut comment_tracker,
                        tab_spaces,
                        whitespace_char,
                    );

                    //> close comment
                        let possible_space = if space_after_comment_starter { " " } else { "" };

                        formatted_lines.push(set_whitespace(
                            &(comment_starter.to_owned() + possible_space + "<\n"),
                            comment_tracker[comment_tracker.len() - 1].depth,
                            whitespace_char,
                        ));

                    //<> move any text after //< to comment on next line
                        // remove bracket from line
                        let comment_contents = &line_no_comment_starter[1..];

                        if !line_is_only_whitepace(comment_contents) {
                            formatted_lines.push(set_whitespace(
                                &(comment_starter.to_owned() + comment_contents + "\n"),
                                comment_tracker[comment_tracker.len() - 1].depth,
                                whitespace_char,
                            ));
                        }
                    //<

                    // remove comment from comment tracker
                    comment_tracker.pop();
                } else if leading_spaces != None {
                    formatted_lines.push(line.to_string() + "\n");
                } else {
                    // all whitespace only lines are set to depth 0
                    formatted_lines.push("\n".to_string());
                }
            //<
        }

        //> turn all lines into one string
            for line in formatted_lines {
                formatted_file.push_str(&line);
            }
        //<

        // if the last char of source str wasn't '\n', don't add the last '\n'
        //> This prevents adding an additional empty line to our output, that wasn't in our input
            if let Some(last_char) = str.chars().last() {
                if last_char != '\n' {
                    // remove the last '\n'
                    formatted_file.pop();
                }
            }

        //<> ensure formatting successful
            if !comment_tracker.is_empty() {
                let err_line = comment_tracker[comment_tracker.len() - 1].line + 1;
                return Err(ScfmtErr::CommentNeverClosed(err_line));
            }
        //<
        Ok(formatted_file)
    }

    /// Runs ``format_str`` on contents of given file.
    pub fn format_file(file: PathBuf) -> Result<(), ScfmtErr> {
        let extenstion = match file.extension() {
            Some(x) => match x.to_str() {
                Some(x) => x,
                None => return Err(ScfmtErr::CantConvertOsString),
            },
            None => return Err(ScfmtErr::IncompatibleFileType),
        };

        let contents = match fs::read_to_string(&file) {
            Ok(x) => x,
            Err(_) => return Err(ScfmtErr::CantReadFileAsString),
        };

        let converted = format_str(&contents, extenstion)?;

        //> write file
            // leave file alone if there was no change
            if converted != contents {
                let mut output = match File::create(file) {
                    Ok(x) => x,
                    Err(_) => return Err(ScfmtErr::CantCreatFile),
                };

                match write!(output, "{}", converted) {
                    Ok(x) => x,
                    Err(_) => return Err(ScfmtErr::CantWriteToFile),
                };
            }
        //<

        Ok(())
    }

    /// Runs ``add_brackets`` on contents of given file.
    pub fn add_brackets_file(file: PathBuf) -> Result<(), ScfmtErr> {
        let extenstion = match file.extension() {
            Some(x) => match x.to_str() {
                Some(x) => x,
                None => return Err(ScfmtErr::CantConvertOsString),
            },
            None => return Err(ScfmtErr::IncompatibleFileType),
        };

        let contents = match fs::read_to_string(&file) {
            Ok(x) => x,
            Err(_) => return Err(ScfmtErr::CantReadFileAsString),
        };

        let converted = add_brackets(&contents, extenstion)?;

        //> write file
            // leave file alone if there was no change
            if converted != contents {
                let mut output = match File::create(file) {
                    Ok(x) => x,
                    Err(_) => return Err(ScfmtErr::CantCreatFile),
                };

                match write!(output, "{}", converted) {
                    Ok(x) => x,
                    Err(_) => return Err(ScfmtErr::CantWriteToFile),
                };
            }
        //<

        Ok(())
    }

    struct CommentDetail {
        line: usize,
        depth: usize,
    }

    fn make_comment_closed_and_open_bracket(line: &str, comment_starter: &str) -> Option<String> {
        let (leading_spaces, line_no_leading_spaces) = chop_off_beginning_spaces(line);

        // remove comment notation if it exists
        let (is_a_comment, _, _line_no_comment_starter) =
            remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

        if !is_a_comment {
            return None;
        }

        let first_half = &line[..leading_spaces.unwrap() + comment_starter.len()];
        let second_half = &line[leading_spaces.unwrap() + comment_starter.len()..];

        Some(String::from(first_half) + "<>" + second_half)
    }

    fn make_comment_open_bracket(line: &str, comment_starter: &str) -> Option<String> {
        // chop off begining spaces
        let (leading_spaces, line_no_leading_spaces) = chop_off_beginning_spaces(line);

        // remove comment notation if it exists
        let (is_a_comment, _, _line_no_comment_starter) =
            remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

        if !is_a_comment {
            return None;
        }

        let first_half = &line[..leading_spaces.unwrap() + comment_starter.len()];
        let second_half = &line[leading_spaces.unwrap() + comment_starter.len()..];

        Some(String::from(first_half) + ">" + second_half)
    }

    fn new_comment_closed_bracket(
        depth: usize,
        comment_starter: &str,
        whitespace_char: char,
    ) -> Option<String> {
        let mut result = String::new();
        for _i in 0..depth {
            result.push(whitespace_char);
        }

        result.push_str(&(String::from(comment_starter) + "<"));
        Some(result)
    }

    fn remove_empty_tail(lines_list: &mut Vec<String>) {
        while !lines_list.is_empty() && line_is_only_whitepace(lines_list.last().unwrap()) {
            lines_list.pop();
        }
    }

    fn end_the_last_structured_comments(
        lines_list: &mut Vec<String>,
        comment_tracker: &mut Vec<CommentDetail>,
        leading_spaces: usize,
        comment_starter: &str,
        whitespace_char: char,
    ) {
        //> remove and remember last empty lines
            let empty_line_count = count_ending_empty_lines(lines_list);
            remove_empty_tail(lines_list);

        //<> add closing bracket comments
            while !comment_tracker.is_empty()
                && leading_spaces <= comment_tracker[comment_tracker.len() - 1].depth
            {
                let close_bracket_line = new_comment_closed_bracket(
                    comment_tracker[comment_tracker.len() - 1].depth,
                    comment_starter,
                    whitespace_char,
                )
                .unwrap();
                lines_list.push(close_bracket_line);
                comment_tracker.pop();
            }

        //<> re-add previously removed whitespace
            append_num_empty_lines(empty_line_count, lines_list);
        //<
    }

    fn pass_a_new_comment_that_we_dont_know_if_its_structured(
        lines_list: &mut Vec<String>,
        comment_tracker: &mut Vec<CommentDetail>,
        leading_spaces: Option<usize>,
        unsure_if_last_comment_was_structured: &mut bool,
        line: &str,
    ) {
        let comment = CommentDetail {
            line: lines_list.len(),
            depth: leading_spaces.unwrap(),
        };

        comment_tracker.push(comment);
        *unsure_if_last_comment_was_structured = true;

        lines_list.push(String::from(line));
    }

    fn count_and_remove_begining_whitespace(line: &str) -> Option<(usize, String)> {
        // chop off begining spaces
        let (leading_whitespace_option, line_no_leading_spaces) = chop_off_beginning_spaces(line);

        leading_whitespace_option.map(|num_leading_whitespace| {
            (num_leading_whitespace, line_no_leading_spaces.to_owned())
        })
    }

    fn last_non_empty_line_before_index(
        index: usize,
        lines_list: &[String],
    ) -> Option<(usize, &str)> {
        for i in (0..index).rev() {
            if !line_is_only_whitepace(&lines_list[i]) {
                return Some((i, &lines_list[i]));
            }
        }

        None
    }

    fn add_open_bracket_to_last_comment(
        lines_list: &mut Vec<String>,
        comment_tracker: &mut [CommentDetail],
        comment_starter: &str,
    ) {
        let mut should_consume_closing_comment = false;

        //> consume any previous now unecessary //<

            let line_of_latest_comment = comment_tracker[comment_tracker.len() - 1].line;

            let last_solid_line_option =
                last_non_empty_line_before_index(line_of_latest_comment, lines_list);

            // if there even could be a //< comment behind the lastest comment
            if let Some((_last_solid_line_index, line_before_open_bracket_comment)) =
                last_solid_line_option
            {
                // chop off begining spaces
                let (leading_spaces, line_no_leading_spaces) =
                    chop_off_beginning_spaces(line_before_open_bracket_comment);

                // remove comment notation if it exists
                let (is_a_comment, _, line_no_comment_opener) =
                    remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

                let latest_comment =
                    match count_and_remove_begining_whitespace(&lines_list[line_of_latest_comment]) {
                        Some(x) => x,
                        None => (0, String::from("")),
                    };

                if is_a_comment
                    && line_no_comment_opener.starts_with('<')
                    && latest_comment.0 == leading_spaces.unwrap()
                {
                    should_consume_closing_comment = true;
                }
            }
        //<
        let line_with_no_bracket = lines_list[line_of_latest_comment].clone();

        if should_consume_closing_comment {
            //> pop everything to the last //<, but remember how to restore what was popped.
                let after_spaces = count_ending_empty_lines(lines_list);
                remove_empty_tail(lines_list);

                // remove the soon to be bracketed comment
                // we'll add it back later
                lines_list.pop();

                let before_spaces = count_ending_empty_lines(lines_list);
                remove_empty_tail(lines_list);

            //<> remove the //<
                lines_list.pop();

            //<> put things back and make add brackets to latest comment

                append_num_empty_lines(before_spaces, lines_list);

                // re-append the latest comment, with added brackets
                lines_list.push(
                    make_comment_closed_and_open_bracket(&line_with_no_bracket, comment_starter)
                        .unwrap(),
                );

                append_num_empty_lines(after_spaces, lines_list);
            //<
        } else {
            // append bracket to latest comment
            lines_list[line_of_latest_comment] =
                make_comment_open_bracket(&line_with_no_bracket, comment_starter).unwrap();
        }
    }

    fn line_is_only_whitepace(str: &str) -> bool {
        for char in str.chars() {
            if char as u32 > 32 {
                return false;
            }
        }
        true
    }

    /// Adds brackets to bracketless structured comments
    pub fn add_brackets(str: &str, filetype: &str) -> Result<String, ScfmtErr> {
        // determine if file compatible
        let comment_starter = match EXTENSION_TO_COMMENT_STARTER_MAP.get(filetype) {
            Some(x) => *x,
            None => return Err(ScfmtErr::IncompatibleFileType),
        };

        // remove existing brackets, so later part of this function doesn't add more on top of existing ones.
        let str = &remove_brackets(str, filetype)?;

        let (whitespace_char, _tab_spaces) = determine_whitespace_type(str);

        let mut comment_tracker: Vec<CommentDetail> = Vec::new();

        let mut lines_list: Vec<String> = Vec::new();
        let mut unsure_if_last_comment_was_structured = true;

        let mut processed_line_count = 0;
        for line in str.lines() {
            // counts how many lines this loop has processed
            processed_line_count += 1;

            // chop off begining spaces
            let (leading_spaces, line_no_leading_spaces) = chop_off_beginning_spaces(line);

            let (is_a_comment, _, _) =
                remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

            match leading_spaces {
                Some(x) => {
                    if is_a_comment {
                        if !comment_tracker.is_empty() {
                            if unsure_if_last_comment_was_structured {
                                if x > comment_tracker[comment_tracker.len() - 1].depth {
                                    // last was structured

                                    add_open_bracket_to_last_comment(
                                        &mut lines_list,
                                        &mut comment_tracker,
                                        comment_starter,
                                    );

                                    pass_a_new_comment_that_we_dont_know_if_its_structured(
                                        &mut lines_list,
                                        &mut comment_tracker,
                                        leading_spaces,
                                        &mut unsure_if_last_comment_was_structured,
                                        line,
                                    );
                                } else {
                                    // last was not structured

                                    comment_tracker.pop();

                                    end_the_last_structured_comments(
                                        &mut lines_list,
                                        &mut comment_tracker,
                                        x,
                                        comment_starter,
                                        whitespace_char,
                                    );

                                    pass_a_new_comment_that_we_dont_know_if_its_structured(
                                        &mut lines_list,
                                        &mut comment_tracker,
                                        leading_spaces,
                                        &mut unsure_if_last_comment_was_structured,
                                        line,
                                    );
                                }
                            } else if x > comment_tracker[comment_tracker.len() - 1].depth {
                                pass_a_new_comment_that_we_dont_know_if_its_structured(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    leading_spaces,
                                    &mut unsure_if_last_comment_was_structured,
                                    line,
                                );
                            } else {
                                end_the_last_structured_comments(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    x,
                                    comment_starter,
                                    whitespace_char,
                                );

                                pass_a_new_comment_that_we_dont_know_if_its_structured(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    leading_spaces,
                                    &mut unsure_if_last_comment_was_structured,
                                    line,
                                );
                            }
                        } else {
                            pass_a_new_comment_that_we_dont_know_if_its_structured(
                                &mut lines_list,
                                &mut comment_tracker,
                                leading_spaces,
                                &mut unsure_if_last_comment_was_structured,
                                line,
                            );
                        }
                    } else if !comment_tracker.is_empty() {
                        if unsure_if_last_comment_was_structured {
                            if x > comment_tracker[comment_tracker.len() - 1].depth {
                                // last was structured

                                add_open_bracket_to_last_comment(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    comment_starter,
                                );
                            } else {
                                // last was not structured

                                comment_tracker.pop();

                                end_the_last_structured_comments(
                                    &mut lines_list,
                                    &mut comment_tracker,
                                    x,
                                    comment_starter,
                                    whitespace_char,
                                );
                            }
                            unsure_if_last_comment_was_structured = false;

                            lines_list.push(String::from(line));
                        } else if x > comment_tracker[comment_tracker.len() - 1].depth {
                            lines_list.push(String::from(line));
                        } else {
                            end_the_last_structured_comments(
                                &mut lines_list,
                                &mut comment_tracker,
                                x,
                                comment_starter,
                                whitespace_char,
                            );

                            // forward the current line
                            lines_list.push(String::from(line));
                        }
                    } else {
                        lines_list.push(String::from(line));
                    }
                }
                None => {
                    lines_list.push("".to_owned());
                }
            }
        }

        //> last comment was not structured, if it was the last non empty line in the String
            if unsure_if_last_comment_was_structured && !comment_tracker.is_empty() {
                comment_tracker.pop();
            }
        //<

        end_the_last_structured_comments(
            &mut lines_list,
            &mut comment_tracker,
            0,
            comment_starter,
            whitespace_char,
        );

        //> turn all lines into one string
            let mut final_string = String::new();
            for line in lines_list {
                final_string.push_str(&line);
                final_string.push('\n');
            }
        //<

        // remove last '\n'
        final_string.pop();

        //> append any missed empty lines, as the .lines() function may have skipped them.
            let line_diff = count_lines(str) - processed_line_count;

            if line_diff > 0 {
                for _ in 0..line_diff {
                    final_string.push('\n');
                }
            }
        //<

        Ok(final_string)
    }

    /// Adds a '_' character in front of any comment brackets. Nullifying any existing bracketed structured comments, without removing any characters.
    ///
    /// This is intended to be run on existing codebases that have not previously been using structured commenting. As brackets may exist in comments that were not intended to be structured comments.
    pub fn null_existing_brackets(str: &str, filetype: &str) -> Result<String, ScfmtErr> {
        // determine if file compatible
        let comment_starter = match EXTENSION_TO_COMMENT_STARTER_MAP.get(filetype) {
            Some(x) => *x,
            None => return Err(ScfmtErr::IncompatibleFileType),
        };

        let (whitespace_char, _tab_spaces) = determine_whitespace_type(str);
        let mut lines_list = Vec::new();
        let mut processed_line_count = 0;
        for line in str.lines() {
            // counts how many lines this loop has processed
            processed_line_count += 1;

            // chop off begining spaces
            let (_leading_spaces, line_no_leading_spaces) = chop_off_beginning_spaces(line);

            // remove comment notation if it exists
            let (is_a_comment, space_after_comment_starter, line_no_comment_starter) =
                remove_comment_notation_if_it_exists(line_no_leading_spaces, comment_starter);

            if is_a_comment && line_no_comment_starter.starts_with('<')
                || line_no_comment_starter.starts_with('>')
            {
                let depth_option = count_and_remove_begining_whitespace(line);
                let depth = match depth_option {
                    Some(x) => x.0,
                    None => 0,
                };

                let potential_space = if space_after_comment_starter { " " } else { "" };

                lines_list.push(add_whitespace(
                    &(comment_starter.to_owned()
                        + potential_space
                        + "_"
                        + &line_no_comment_starter),
                    depth,
                    whitespace_char,
                ));
            } else if line_is_only_whitepace(line) {
                lines_list.push("".to_owned());
            } else {
                lines_list.push(line.to_owned());
            }
        }

        //> turn all lines into one string
            let mut final_string = String::new();
            for line in lines_list {
                final_string.push_str(&line);
                final_string.push('\n');
            }
        //<

        // remove last '\n'
        final_string.pop();

        //> append any missed empty lines, as the .lines() function may have skipped them.
            let line_diff = count_lines(str) - processed_line_count;

            if line_diff > 0 {
                for _ in 0..line_diff {
                    final_string.push('\n');
                }
            }
        //<

        Ok(final_string)
    }

    fn count_lines(str: &str) -> usize {
        if str.is_empty() {
            return 0;
        }

        let mut count = 1;
        for c in str.chars() {
            if c == '\n' {
                count += 1;
            }
        }
        count
    }

    /// Runs ``remove_brackets`` on contents of given file
    pub fn remove_brackets_file(file: PathBuf) -> Result<(), ScfmtErr> {
        let extenstion = match file.extension() {
            Some(x) => match x.to_str() {
                Some(x) => x,
                None => return Err(ScfmtErr::CantConvertOsString),
            },
            None => return Err(ScfmtErr::IncompatibleFileType),
        };

        let contents = match fs::read_to_string(&file) {
            Ok(x) => x,
            Err(_) => return Err(ScfmtErr::CantReadFileAsString),
        };

        let converted = remove_brackets(&contents, extenstion)?;

        //> write file
            // leave file alone if there was no change
            if converted != contents {
                let mut output = match File::create(file) {
                    Ok(x) => x,
                    Err(_) => return Err(ScfmtErr::CantCreatFile),
                };

                match write!(output, "{}", converted) {
                    Ok(x) => x,
                    Err(_) => return Err(ScfmtErr::CantWriteToFile),
                };
            }
        //<

        Ok(())
    }

    /// Runs ``null_existing_brackets`` on contents of given file
    pub fn null_existing_brackets_file(file: PathBuf) -> Result<(), ScfmtErr> {
        let extenstion = match file.extension() {
            Some(x) => match x.to_str() {
                Some(x) => x,
                None => return Err(ScfmtErr::CantConvertOsString),
            },
            None => return Err(ScfmtErr::IncompatibleFileType),
        };

        let contents = match fs::read_to_string(&file) {
            Ok(x) => x,
            Err(_) => return Err(ScfmtErr::CantReadFileAsString),
        };

        let converted = null_existing_brackets(&contents, extenstion)?;

        //> write file
            // leave file alone if there was no change
            if converted != contents {
                let mut output = match File::create(file) {
                    Ok(x) => x,
                    Err(_) => return Err(ScfmtErr::CantCreatFile),
                };

                match write!(output, "{}", converted) {
                    Ok(x) => x,
                    Err(_) => return Err(ScfmtErr::CantWriteToFile),
                };
            }
        //<

        Ok(())
    }

    fn line_is_a_comment(str: &str, comment_starter: &str) -> bool {
        match count_and_remove_begining_whitespace(str) {
            Some(x) => {
                let comment_starter_with_space = comment_starter.to_owned() + " ";
                let str = x.1;

                if str.starts_with(&comment_starter_with_space) {
                    true
                } else {
                    str.starts_with(comment_starter)
                }
            }
            None => false,
        }
    }

    fn remove_comment_starter(str: &str, comment_starter: &str) -> String {
        match count_and_remove_begining_whitespace(str) {
            Some(x) => {
                let str = x.1;

                let mut line_no_comment_starter = "";
                let comment_starter_with_space = comment_starter.to_owned() + " ";
                if str.starts_with(&comment_starter_with_space) {
                    line_no_comment_starter = &str[comment_starter.len() + 1..];
                } else if let Some(stripped) = str.strip_prefix(comment_starter) {
                    line_no_comment_starter = stripped;
                }

                line_no_comment_starter.to_string()
            }
            None => str.to_owned(),
        }
    }

    fn count_ending_empty_lines(lines_list: &[String]) -> usize {
        let mut count = 0;
        for i in (0..lines_list.len()).rev() {
            if !line_is_only_whitepace(&lines_list[i]) {
                break;
            }
            count += 1;
        }

        count
    }

    fn append_num_empty_lines(num: usize, lines_list: &mut Vec<String>) {
        for _ in 0..num {
            lines_list.push("".to_owned());
        }
    }

    /// Converts bracketed structured comments into bracketless structured comments
    ///
    /// Becuase bracketless structured comments rely soley on indentation to show what lines they are talking about, this function formats the input str before removing bracket comments. To ensure structured comment information is not lost.
    pub fn remove_brackets(str: &str, filetype: &str) -> Result<String, ScfmtErr> {
        // determine if file compatible
        let comment_starter = match EXTENSION_TO_COMMENT_STARTER_MAP.get(filetype) {
            Some(x) => *x,
            None => return Err(ScfmtErr::IncompatibleFileType),
        };

        let mut lines_list: Vec<String> = Vec::new();

        //format str before removing brackets, to ensure their information is not lost.
        let str = &format_str(str, filetype)?;

        let (whitespace_char, _tab_spaces) = determine_whitespace_type(str);

        let mut formatted_str = String::new();
        let mut processed_line_count = 0;

        for line in str.lines() {
            // counts how many lines this loop has processed
            processed_line_count += 1;

            let line_no_leading_whitespace;
            let leading_whitespace;

            if let Some(x) = count_and_remove_begining_whitespace(line) {
                leading_whitespace = x.0;
                line_no_leading_whitespace = &x.1;

                if line_is_a_comment(line_no_leading_whitespace, comment_starter) {
                    let line_no_comment_starter =
                        remove_comment_starter(line_no_leading_whitespace, comment_starter);

                    if let Some(line_no_brackets) = line_no_comment_starter.strip_prefix("<>") {
                        lines_list.push(
                            add_whitespace(
                                &(comment_starter.to_owned() + line_no_brackets),
                                leading_whitespace,
                                whitespace_char,
                            ) + "\n",
                        );
                    } else if let Some(line_no_brackets) = line_no_comment_starter.strip_prefix('>')
                    {
                        lines_list.push(
                            add_whitespace(
                                &(comment_starter.to_owned() + line_no_brackets),
                                leading_whitespace,
                                whitespace_char,
                            ) + "\n",
                        );
                    } else if line_no_comment_starter.starts_with('<') {
                        // remove line by not adding it to output
                        continue;
                    } else {
                        lines_list.push(line.to_owned() + "\n");
                    }
                } else {
                    lines_list.push(line.to_owned() + "\n");
                }
            } else {
                lines_list.push("\n".to_owned());
            }
        }

        //> turn all lines into one string
            for line in lines_list {
                formatted_str.push_str(&line);
            }
        //<

        // remove last '\n'
        formatted_str.pop();

        //> append any missed empty lines, as the .lines() function may have skipped them.
            let line_diff = count_lines(str) - processed_line_count;

            if line_diff > 0 {
                for _ in 0..line_diff {
                    formatted_str.push('\n');
                }
            }
        //<

        Ok(formatted_str)
    }
}
