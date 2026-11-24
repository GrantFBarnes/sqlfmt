use crate::configuration::{ConfigCase, ConfigTab, Configuration};
use crate::token::*;

pub fn get_formatted_sql(config: &Configuration, input_sql: String) -> String {
    let mut state: FormatState = FormatState::new();

    let input_tokens: Vec<Token> = get_sql_tokens(config, input_sql);
    for i in 0..input_tokens.len() {
        let input_token: &Token = &input_tokens[i];
        let prev_input_token: Option<&Token> = if i > 0 { input_tokens.get(i - 1) } else { None };
        let next_input_token: Option<&Token> = input_tokens.get(i + 1);

        if state.continue_on_input_whitespace(
            input_token,
            prev_input_token,
            next_input_token,
            config,
        ) {
            continue;
        }
        state.increase_paren_stack(input_token);
        state.decrease_indent_stack(input_token);
        state.add_pre_space(input_token, prev_input_token, next_input_token, config);
        state.set_previous_comment_pre_space(config);
        state.push(input_token.clone());
        state.increase_indent_stack(input_token);
        state.decrease_paren_stack(input_token);
        state.align_text_groups(config, input_token);
    }

    state.get_result(config)
}

struct FormatState {
    tokens: Vec<Token>,
    prefix: Option<String>,
    indent_stack: Vec<Token>,
    paren_stack: Vec<ParenCategory>,
    // block opener (BEGIN/DO/CASE/THEN/ELSE) closed by the most recent END
    end_block_opener: Option<Token>,
}

impl FormatState {
    fn new() -> FormatState {
        FormatState {
            tokens: vec![],
            prefix: None,
            indent_stack: vec![],
            paren_stack: vec![],
            end_block_opener: None,
        }
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn continue_on_input_whitespace(
        &mut self,
        input_token: &Token,
        prev_input_token: Option<&Token>,
        next_input_token: Option<&Token>,
        config: &Configuration,
    ) -> bool {
        match input_token.category {
            TokenCategory::NewLine => {
                // keep user input newlines as is
                if !config.newlines {
                    return false;
                }

                // keep user input pre-space if after newline
                if let Some(next_token) = next_input_token
                    && next_token
                        .behavior
                        .contains(&TokenBehavior::PreSpaceKeepInput)
                    && let Some(prev_token) = self.tokens.last()
                {
                    if prev_token.behavior.contains(&TokenBehavior::NewLineAfterX2) {
                        self.push(Token::new_newline());
                        self.push(Token::new_newline());
                    } else {
                        self.push(Token::new_newline());
                    }
                    return true;
                }

                // ignore all other user input newlines
                true
            }
            TokenCategory::WhiteSpace => {
                // define and keep user input space as prefix if not found
                if self.prefix.is_none() {
                    self.prefix = Some(input_token.value.clone());
                    self.push(input_token.clone());
                    return true;
                }

                // keep user input pre-space if after newline
                if let Some(next_token) = next_input_token
                    && next_token
                        .behavior
                        .contains(&TokenBehavior::PreSpaceKeepInput)
                    && prev_input_token.is_some_and(|t| t.category == TokenCategory::NewLine)
                {
                    let prev_token: Option<&Token> = self.tokens.last();
                    if prev_token.is_none_or(|t| t.category != TokenCategory::NewLine) {
                        if prev_token
                            .is_some_and(|t| t.behavior.contains(&TokenBehavior::NewLineAfterX2))
                        {
                            self.push(Token::new_newline());
                            self.push(Token::new_newline());
                        } else {
                            self.push(Token::new_newline());
                        }
                    }
                    self.push(input_token.clone());
                    return true;
                }

                // ignore all other user input spaces
                true
            }
            _ => {
                if self.prefix.is_none() {
                    self.prefix = Some(String::new());
                }
                false
            }
        }
    }

    fn increase_paren_stack(&mut self, token: &Token) {
        if token.category != TokenCategory::ParenOpen {
            return;
        }

        if let Some(prev_token) = self.tokens.last() {
            match prev_token.category {
                TokenCategory::XmlMethod => {
                    self.paren_stack.push(ParenCategory::Space0Newline0);
                    return;
                }
                TokenCategory::DataType | TokenCategory::Method | TokenCategory::Identifier => {
                    self.paren_stack.push(ParenCategory::Space0Newline1);
                    return;
                }
                _ => (),
            }
        }

        self.paren_stack.push(ParenCategory::Space1Newline1);
    }

    fn decrease_paren_stack(&mut self, token: &Token) {
        if token.category == TokenCategory::ParenClose {
            self.paren_stack.pop();
        }
    }

    fn align_text_groups(&mut self, config: &Configuration, token: &Token) {
        if !config.align_text_groups {
            return;
        }

        if token.category != TokenCategory::ParenClose {
            return;
        }

        let Some(lines) = self.get_paren_text_group_lines() else {
            return;
        };

        self.pad_text_groups(&lines);
    }

    // collect the text groups on each line of the just-closed paren
    // returns None if the paren contains indented formatting, which is not aligned
    fn get_paren_text_group_lines(&self) -> Option<Vec<Vec<TextGroup>>> {
        let mut paren_count: usize = 1;
        let mut indent_len: Option<usize> = None;
        let mut last_whitespace_position: Option<usize> = None;
        let mut current_group: Option<TextGroup> = None;
        let mut current_line: Vec<TextGroup> = vec![];
        let mut lines: Vec<Vec<TextGroup>> = vec![];

        // loop backwards until previous newline outside paren set
        for i in (0..self.tokens.len() - 1).rev() {
            let prev_token: &Token = &self.tokens[i];

            if paren_count == 0 {
                break;
            }

            if prev_token.category == TokenCategory::NewLine {
                // check if indented formatting inside paren
                if self.tokens.get(i + 2).is_some() {
                    let current_indent_len: usize = self.tokens[i + 2].len();
                    if current_indent_len > 0 {
                        if indent_len.is_none() {
                            indent_len = Some(current_indent_len);
                        }
                        if current_indent_len > indent_len.unwrap() {
                            // inner paren contains indented formatting
                            // do not align groups
                            return None;
                        }
                    }
                }

                if !current_line.is_empty() {
                    current_line.reverse();
                    lines.push(current_line);
                }
                last_whitespace_position = None;
                current_group = None;
                current_line = vec![];
            } else {
                match prev_token.category {
                    TokenCategory::ParenOpen => paren_count -= 1,
                    TokenCategory::ParenClose => paren_count += 1,
                    TokenCategory::Comma if paren_count == 1 => {
                        if let Some(cg) = current_group {
                            current_line.push(cg);
                        }
                        current_line.push(TextGroup {
                            value: prev_token.value.clone(),
                            len: prev_token.len(),
                            post_whitespace_position: last_whitespace_position,
                        });
                        current_group = None;
                        continue;
                    }
                    TokenCategory::WhiteSpace if paren_count == 1 => {
                        if let Some(cg) = current_group {
                            current_line.push(cg);
                            merge_keyword_groupings(&mut current_line);

                            current_group = None;
                        }
                        last_whitespace_position = Some(i);
                        continue;
                    }
                    _ => (),
                }

                if current_group.is_none() {
                    current_group = Some(TextGroup {
                        value: String::new(),
                        len: 0,
                        post_whitespace_position: last_whitespace_position,
                    });
                }
                let mut new_current_group: TextGroup = current_group.unwrap();
                new_current_group.value =
                    format!("{}{}", prev_token.value, new_current_group.value);
                new_current_group.len += prev_token.len();
                current_group = Some(new_current_group);
            }
        }

        Some(lines)
    }

    // pad the whitespace after each text group so the groups line up in columns
    fn pad_text_groups(&mut self, lines: &[Vec<TextGroup>]) {
        // find max len in each group
        let max_groups_per_line: usize = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let mut max_len_in_each_group: Vec<usize> = vec![0; max_groups_per_line];
        for line in lines {
            for group_idx in 0..line.len() {
                if line[group_idx].len > max_len_in_each_group[group_idx] {
                    max_len_in_each_group[group_idx] = line[group_idx].len;
                }
            }
        }

        // adjust post whitespace len to align each group
        for line in lines {
            for group_idx in 0..line.len() {
                if line[group_idx].post_whitespace_position.is_some() {
                    let post_whitespace_position: usize =
                        line[group_idx].post_whitespace_position.unwrap();
                    let max_len: usize = max_len_in_each_group[group_idx];
                    let len: usize = line[group_idx].len;
                    let whitespace_len: usize = 1 + max_len - len;
                    self.tokens[post_whitespace_position].value = " ".repeat(whitespace_len);
                }
            }
        }
    }

    fn add_pre_space(
        &mut self,
        token: &Token,
        prev_input_token: Option<&Token>,
        next_input_token: Option<&Token>,
        config: &Configuration,
    ) {
        if self
            .tokens
            .last()
            .is_none_or(|t| t.category == TokenCategory::WhiteSpace)
        {
            return;
        }

        if token.behavior.contains(&TokenBehavior::NoWhiteSpaceBefore) {
            return;
        }

        if token
            .behavior
            .contains(&TokenBehavior::NoSpaceAroundIfNotProvidedInput)
            && prev_input_token.is_none_or(|t| {
                t.category != TokenCategory::WhiteSpace && t.category != TokenCategory::NewLine
            })
        {
            return;
        }

        if config.newlines {
            self.insert_newline(token, config);
            self.add_pre_newline(token, next_input_token);
            self.remove_extra_newline(token, config);
        }

        let prev_token: &Token = self
            .tokens
            .last()
            .expect("should always have a previous token");

        if token
            .behavior
            .contains(&TokenBehavior::NoSpaceBeforeIfStartOfNewLine)
            && prev_token.category == TokenCategory::NewLine
        {
            return;
        }

        match token.category {
            TokenCategory::Delimiter => {
                if prev_token.upper != "DELIMITER" {
                    return;
                }
            }
            TokenCategory::ParenOpen
                if let Some(paren) = self.paren_stack.last()
                    && (paren == &ParenCategory::Space0Newline0
                        || paren == &ParenCategory::Space0Newline1) =>
            {
                return;
            }
            _ => (),
        }

        if prev_token.behavior.contains(&TokenBehavior::NoSpaceAfter) {
            return;
        }

        if prev_token
            .behavior
            .contains(&TokenBehavior::NoSpaceAroundIfNotProvidedInput)
            && prev_input_token.is_none_or(|t| {
                t.category != TokenCategory::WhiteSpace && t.category != TokenCategory::NewLine
            })
        {
            return;
        }

        if prev_token.category == TokenCategory::NewLine {
            for t in self.get_newline_pre_space_tokens(config) {
                self.push(t);
            }
            return;
        }

        if token.behavior.contains(&TokenBehavior::NoSpaceBefore) {
            return;
        }

        self.push(Token::new_whitespace(String::from(" ")));
    }

    fn insert_newline(&mut self, token: &Token, config: &Configuration) {
        // a trailing comment should not cause the code before it to wrap
        if token.category == TokenCategory::Comment
            || self
                .tokens
                .last()
                .is_some_and(|t| t.category == TokenCategory::Comment)
        {
            return;
        }

        if self.get_current_line_length(token) > config.chars.into() {
            self.insert_newline_after_last_operator(self.tokens.len(), config);
        }
    }

    fn get_current_line_length(&self, token: &Token) -> usize {
        let mut line_length: usize = token.len();
        for i in (0..self.tokens.len()).rev() {
            let prev_token: &Token = &self.tokens[i];
            match prev_token.category {
                TokenCategory::NewLine => break,
                _ => line_length += prev_token.len(),
            }
        }
        line_length
    }

    fn add_pre_newline(&mut self, token: &Token, next_input_token: Option<&Token>) {
        if self.tokens.is_empty() {
            return;
        }

        if self.paren_stack.contains(&ParenCategory::Space0Newline0) {
            return;
        }

        if token.behavior.contains(&TokenBehavior::NoWhiteSpaceBefore) {
            return;
        }

        if token.behavior.contains(&TokenBehavior::NoNewLineBefore) {
            return;
        }

        let prev1_token: &Token = self
            .tokens
            .last()
            .expect("should always have a previous token");
        let prev3_token: Option<&Token> = self.tokens.iter().nth_back(2);

        // keep 'END IF' together when END closed an 'IF ... THEN [... ELSE]' block
        if token.upper == "IF"
            && prev1_token.upper == "END"
            && self
                .end_block_opener
                .as_ref()
                .is_some_and(|t| t.upper == "THEN" || t.upper == "ELSE")
        {
            return;
        }

        if token
            .behavior
            .contains(&TokenBehavior::NoNewLineBeforeUnlessMatch)
            && prev1_token.category != token.category
        {
            return;
        }

        if prev1_token
            .behavior
            .contains(&TokenBehavior::NewLineAfterX2IfCombined)
            && let Some(p3t) = prev3_token
            && p3t
                .behavior
                .contains(&TokenBehavior::NewLineAfterX2IfCombined)
        {
            self.push(Token::new_newline());
            self.push(Token::new_newline());
            return;
        }

        if prev1_token.behavior.contains(&TokenBehavior::NewLineAfter)
            && token.category != TokenCategory::Delimiter
        {
            self.push(Token::new_newline());
            return;
        }

        if prev1_token
            .behavior
            .contains(&TokenBehavior::NewLineAfterX2)
        {
            self.push(Token::new_newline());
            self.push(Token::new_newline());
            return;
        }

        if prev1_token.category == TokenCategory::ParenOpen {
            if token.category != TokenCategory::ParenClose {
                self.push(Token::new_newline());
            }
            return;
        }

        if token.behavior.contains(&TokenBehavior::NewLineBefore) {
            self.push(Token::new_newline());
            return;
        }

        if token
            .behavior
            .contains(&TokenBehavior::NewLineBeforeIfNotMethodCall)
            && next_input_token.is_none_or(|t| t.category != TokenCategory::ParenOpen)
        {
            self.push(Token::new_newline());
            return;
        }

        if token
            .behavior
            .contains(&TokenBehavior::NewLineBeforeIfNotAfterKeywordOrMethod)
            && prev1_token.category != TokenCategory::Keyword
            && prev1_token.category != TokenCategory::Method
        {
            self.push(Token::new_newline());
            return;
        }

        if token
            .behavior
            .contains(&TokenBehavior::NewLineBeforeIfNotAfterWithin)
            && prev1_token.upper != "WITHIN"
        {
            self.push(Token::new_newline());
            return;
        }

        if token
            .behavior
            .contains(&TokenBehavior::NewLineBeforeIfNotAfterEvent)
            && prev1_token.category != TokenCategory::DataType
            && prev1_token.category != TokenCategory::Event
            && prev3_token.is_none_or(|t| t.category != TokenCategory::Event)
        {
            self.push(Token::new_newline());
            return;
        }

        if prev1_token
            .behavior
            .contains(&TokenBehavior::NewLineAfterIfNotAfterKeyword)
            && let Some(p3t) = prev3_token
            && p3t.category != TokenCategory::Keyword
        {
            self.push(Token::new_newline());
            return;
        }

        if prev3_token.is_some_and(|t| t.behavior.contains(&TokenBehavior::NewLineAfterSkip))
            || (prev1_token.category == TokenCategory::ParenClose
                && prev3_token.is_some_and(|t| t.category == TokenCategory::ParenOpen)
                && self
                    .tokens
                    .iter()
                    .nth_back(4)
                    .is_some_and(|t| t.behavior.contains(&TokenBehavior::NewLineAfterSkip)))
        {
            self.push(Token::new_newline());
        }
    }

    fn remove_extra_newline(&mut self, token: &Token, config: &Configuration) {
        if token.category == TokenCategory::ParenClose {
            self.collapse_paren(config);
        } else if token.behavior.contains(&TokenBehavior::NoNewLineBeforeX2) {
            self.remove_double_newline();
        } else if token.category == TokenCategory::Delimiter {
            self.remove_double_newline_between_delimiter_lines();
        } else if token.upper == "FROM" {
            self.join_is_distinct_from();
        }
    }

    // collapse the just-closed paren onto one line if short enough
    fn collapse_paren(&mut self, config: &Configuration) {
        let mut paren_count: usize = 1;
        let mut inner_token_count: usize = 0;
        let mut collapsed_line_len: usize = 1;
        let mut collapsed_inner_len: usize = 1;
        let mut last_operator_len_after: Option<usize> = None;
        let mut last_operator_position: Option<usize> = None;
        let mut positions_to_remove: Vec<usize> = vec![];
        let mut positions_to_add_space: Vec<usize> = vec![];

        // loop backwards until previous newline outside paren set
        for i in (0..self.tokens.len()).rev() {
            let prev_token: &Token = &self.tokens[i];

            if paren_count == 0 {
                // no longer inside paren set in question
                match prev_token.category {
                    TokenCategory::NewLine => break,
                    TokenCategory::Operator if last_operator_len_after.is_none() => {
                        last_operator_len_after = Some(collapsed_line_len);
                        last_operator_position = Some(i);
                    }
                    _ => (),
                }
            } else {
                // still inside paren set in question
                match prev_token.category {
                    TokenCategory::ParenOpen => paren_count -= 1,
                    TokenCategory::ParenClose => paren_count += 1,
                    TokenCategory::WhiteSpace => {
                        if let Some(pt) = i.checked_sub(1).and_then(|j| self.tokens.get(j))
                            && (pt.category == TokenCategory::NewLine
                                || pt.category == TokenCategory::WhiteSpace)
                        {
                            positions_to_remove.push(i);
                            continue;
                        }
                    }
                    TokenCategory::NewLine => {
                        positions_to_remove.push(i);

                        if let Some(pnwt) = self.get_prev_nonwhitespace_token(i)
                            && pnwt.category != TokenCategory::ParenOpen
                            && let Some(nnwt) = self.get_next_nonwhitespace_token(i)
                            && nnwt.category != TokenCategory::ParenClose
                        {
                            positions_to_add_space.push(i);

                            // increase length for space that would replace the newline if collapsed
                            collapsed_line_len += 1;
                            collapsed_inner_len += 1;
                        }
                        continue;
                    }
                    TokenCategory::Comment if prev_token.value.starts_with("--") => {
                        // a single-line comment ends at its newline, so collapsing
                        // would pull the following tokens into the comment
                        return;
                    }
                    _ => inner_token_count += 1,
                }
                collapsed_inner_len += prev_token.len();
            }

            collapsed_line_len += prev_token.len();
        }

        // determine whether to collapse paren
        if inner_token_count <= 1 // paren is empty or just one inner token
            || collapsed_line_len <= config.chars.into() // collapsed line is short enough
            || last_operator_len_after.is_some_and(|len_after| {
                collapsed_line_len - len_after <= config.chars.into() // line up to operator is short enough
                    && collapsed_inner_len + self.get_newline_pre_space_len(config) // collapsed paren plus indent is short enough
                        <= config.chars.into()
            })
        {
            for p in positions_to_remove {
                self.tokens.remove(p);
                if positions_to_add_space.contains(&p) {
                    self.tokens
                        .insert(p, Token::new_whitespace(String::from(" ")));
                }
            }
        }

        // determine whether to insert newline after last operator
        if collapsed_line_len > config.chars.into() // collapsed line was too long
            && last_operator_len_after
                .is_some_and(|len_after| {
                    collapsed_line_len - len_after <= config.chars.into() // line up to operator is short enough
            })
        {
            self.insert_newline_after_last_operator(last_operator_position.unwrap() + 1, config);
        }
    }

    // remove a double newline before the current token
    fn remove_double_newline(&mut self) {
        if self.tokens.len() < 2 {
            return;
        }

        if self.tokens[self.tokens.len() - 1].category != TokenCategory::NewLine {
            return;
        }

        if self.tokens[self.tokens.len() - 2].category != TokenCategory::NewLine {
            return;
        }

        self.tokens.pop();
    }

    // remove double newline for two consecutive single delimiter lines
    fn remove_double_newline_between_delimiter_lines(&mut self) {
        let mut prev_newline_positions: Vec<usize> = vec![];
        let mut prev_endline_tokens: Vec<&Token> = vec![];
        for i in (1..self.tokens.len()).rev() {
            if self.tokens[i].category == TokenCategory::NewLine {
                prev_newline_positions.push(i);
                prev_endline_tokens.push(&self.tokens[i - 1]);
                if prev_newline_positions.len() >= 3 {
                    break;
                }
            }
        }

        // need at least two newlines to remove extra
        if prev_newline_positions.len() < 2 {
            return;
        }

        // last two newlines need to be next to each other
        if prev_newline_positions[0] != prev_newline_positions[1] + 1 {
            return;
        }

        // need previous line to end in delimiter
        if prev_endline_tokens[1].category != TokenCategory::Delimiter {
            return;
        }

        if prev_endline_tokens.len() == 2
            || prev_endline_tokens[2]
                .behavior
                .contains(&TokenBehavior::NoNewLineAfterX2Skip)
        {
            self.tokens.remove(prev_newline_positions[0]);
        }
    }

    // remove newline for 'IS [NOT] DISTINCT FROM'
    fn join_is_distinct_from(&mut self) {
        if self.tokens.len() < 4 {
            return;
        }

        if self.tokens[self.tokens.len() - 1].category != TokenCategory::NewLine {
            return;
        }

        if self.tokens[self.tokens.len() - 2].upper != "DISTINCT" {
            return;
        }

        if self.tokens[self.tokens.len() - 3].category != TokenCategory::WhiteSpace {
            return;
        }

        if self.tokens[self.tokens.len() - 4].upper != "IS"
            && self.tokens[self.tokens.len() - 4].upper != "NOT"
        {
            return;
        }

        self.tokens.pop();
    }

    fn get_prev_nonwhitespace_token(&self, pos: usize) -> Option<&Token> {
        for i in (0..std::cmp::min(pos, self.tokens.len())).rev() {
            let prev_token: &Token = &self.tokens[i];
            match prev_token.category {
                TokenCategory::WhiteSpace => continue,
                TokenCategory::NewLine => continue,
                _ => return Some(prev_token),
            }
        }
        None
    }

    fn get_next_nonwhitespace_token(&self, pos: usize) -> Option<&Token> {
        for i in pos + 1..self.tokens.len() {
            let next_token: &Token = &self.tokens[i];
            match next_token.category {
                TokenCategory::WhiteSpace => continue,
                TokenCategory::NewLine => continue,
                _ => return Some(next_token),
            }
        }
        None
    }

    fn insert_newline_after_last_operator(&mut self, pos: usize, config: &Configuration) {
        for i in (0..std::cmp::min(pos, self.tokens.len())).rev() {
            match self.tokens[i].category {
                TokenCategory::NewLine => return,
                TokenCategory::Operator => {
                    if i == self.tokens.len() - 1 {
                        self.tokens.push(Token::new_newline());
                        return;
                    }

                    if self.tokens[i].value == "*"
                        && self.tokens[i + 1].category == TokenCategory::ParenClose
                    {
                        return;
                    }

                    if self.tokens[i + 1].category == TokenCategory::WhiteSpace {
                        self.tokens.remove(i + 1);
                    }

                    self.tokens.insert(i + 1, Token::new_newline());

                    let mut newline_pre_space_tokens: Vec<Token> =
                        self.get_newline_pre_space_tokens(config);
                    newline_pre_space_tokens.reverse();
                    for t in newline_pre_space_tokens {
                        self.tokens.insert(i + 2, t);
                    }

                    return;
                }
                _ => continue,
            }
        }
    }

    fn get_newline_pre_space_len(&self, config: &Configuration) -> usize {
        let mut size: usize = 0;
        for t in self.get_newline_pre_space_tokens(config) {
            size += t.len();
        }
        size
    }

    fn get_newline_pre_space_tokens(&self, config: &Configuration) -> Vec<Token> {
        let mut result: Vec<Token> = vec![];

        result.push(Token::new_whitespace(if let Some(prefix) = &self.prefix {
            prefix.clone()
        } else {
            String::new()
        }));

        result.push(Token::new_whitespace(match config.tabs {
            ConfigTab::Tab => "\t".repeat(self.indent_stack.len()),
            ConfigTab::Space(c) => " ".repeat(c as usize * self.indent_stack.len()),
        }));

        result
    }

    fn increase_indent_stack(&mut self, token: &Token) {
        if token.behavior.contains(&TokenBehavior::IncreaseIndent) {
            self.indent_stack.push(token.clone());
            return;
        }

        if token
            .behavior
            .contains(&TokenBehavior::IncreaseIndentIfNotAfterKeyword)
            && self.tokens.iter().nth_back(2).is_none_or(|t| {
                t.category != TokenCategory::Keyword && t.category != TokenCategory::DataType
            })
        {
            self.indent_stack.push(token.clone());
            return;
        }

        if token
            .behavior
            .contains(&TokenBehavior::IncreaseIndentIfStandAlone)
            && self.indent_stack.last().is_none_or(|t| {
                !t.behavior
                    .contains(&TokenBehavior::IncreaseIndentIfStandAlone)
            })
        {
            self.indent_stack.push(token.clone());
        }
    }

    fn decrease_indent_stack(&mut self, token: &Token) {
        let is_end: bool = token.upper == "END";
        if is_end {
            self.end_block_opener = None;
        }

        if self.indent_stack.is_empty() {
            return;
        }

        let decrease_until: Vec<&str> = REQUIRED_TO_DECREASE
            .iter()
            .filter(|(_, closers)| closers.contains(&token.upper.as_str()))
            .map(|(opener, _)| *opener)
            .collect();

        if !decrease_until.is_empty() {
            loop {
                let top: Option<Token> = self.indent_stack.pop();
                if top.is_none() {
                    return;
                }
                let top: Token = top.unwrap();

                if decrease_until.contains(&top.upper.as_str()) {
                    if is_end {
                        self.end_block_opener = Some(top);
                    }
                    return;
                }

                if REQUIRED_TO_DECREASE
                    .iter()
                    .any(|(opener, _)| *opener == top.upper)
                {
                    self.indent_stack.push(top);
                    return;
                }
            }
        }

        if token.behavior.contains(&TokenBehavior::DecreaseIndent) {
            loop {
                let top: Option<Token> = self.indent_stack.pop();
                if top.is_none() {
                    return;
                }
                let top: Token = top.unwrap();

                if REQUIRED_TO_DECREASE
                    .iter()
                    .any(|(opener, _)| *opener == top.upper)
                {
                    self.indent_stack.push(top);
                    return;
                }

                if top.behavior.contains(&TokenBehavior::DecreaseIndent) {
                    return;
                }
            }
        }
    }

    fn set_previous_comment_pre_space(&mut self, config: &Configuration) {
        if !config.comment_pre_space {
            return;
        }

        let token_count: usize = self.tokens.len();
        if token_count <= 2 {
            return;
        }

        if self.tokens[token_count - 1].category != TokenCategory::WhiteSpace
            || self.tokens[token_count - 2].category != TokenCategory::WhiteSpace
            || self.tokens[token_count - 3].category != TokenCategory::NewLine
        {
            return;
        }

        self.set_previous_line_comment_pre_space(
            token_count - 3,
            self.tokens[token_count - 1].value.clone(),
        );
    }

    fn set_previous_line_comment_pre_space(&mut self, newline_index: usize, pre_space: String) {
        if newline_index <= 3 {
            return;
        }

        if self.tokens[newline_index - 1].category != TokenCategory::Comment
            || self.tokens[newline_index - 2].category != TokenCategory::WhiteSpace
            || self.tokens[newline_index - 3].category != TokenCategory::WhiteSpace
            || self.tokens[newline_index - 4].category != TokenCategory::NewLine
        {
            return;
        }

        self.tokens[newline_index - 2].value = pre_space.clone();

        self.set_previous_line_comment_pre_space(newline_index - 4, pre_space);
    }

    fn get_result(&self, config: &Configuration) -> String {
        let mut result: String = String::new();
        for i in 0..self.tokens.len() {
            let token: &Token = &self.tokens[i];

            let mut token_value: String = token.value.clone();

            match token.category {
                TokenCategory::Keyword
                | TokenCategory::DataType
                | TokenCategory::Event
                | TokenCategory::Method => match config.case {
                    ConfigCase::Uppercase => token_value = token.upper.clone(),
                    ConfigCase::Lowercase => token_value = token_value.to_lowercase(),
                    ConfigCase::Unchanged => (),
                },
                TokenCategory::XmlMethod => {
                    if result.ends_with(FULL_STOP)
                        && self
                            .tokens
                            .get(i + 1)
                            .is_some_and(|t| t.category == TokenCategory::ParenOpen)
                    {
                        token_value = token_value.to_lowercase();
                    }
                }
                TokenCategory::NewLine => trim_end_line_whitespace(&mut result),
                _ => (),
            }

            result.push_str(token_value.as_str());
        }
        trim_end_line_whitespace(&mut result);
        result
    }
}

// trim in place, as rebuilding the string for every line is quadratic
fn trim_end_line_whitespace(value: &mut String) {
    let trimmed_len: usize = value
        .trim_end_matches(|c: char| c.is_whitespace() && c != NEW_LINE)
        .len();
    value.truncate(trimmed_len);
}

struct TextGroup {
    value: String,
    len: usize,
    post_whitespace_position: Option<usize>,
}

// keyword sequences kept together as a single text group when aligning
const KEYWORD_GROUPINGS: [&[&str]; 6] = [
    &["NOT", "NULL"],
    &["SET", "NULL"],
    &["SET", "DEFAULT"],
    &["NO", "ACTION"],
    &["ON", "DELETE"],
    &["ON", "UPDATE"],
];

// merge the most recent groups of a line into one if they form a keyword grouping
// lines are built backwards, so the last group in the line is the first word
fn merge_keyword_groupings(line: &mut Vec<TextGroup>) {
    for grouping in &KEYWORD_GROUPINGS {
        let mut group_found: bool = true;
        for g in 0..grouping.len() {
            if g > line.len() - 1 {
                group_found = false;
                break;
            }
            if grouping[g] != line[line.len() - g - 1].value.to_uppercase() {
                group_found = false;
                break;
            }
        }

        if group_found {
            let mut merge_group: TextGroup = line.pop().unwrap();
            for _ in 0..grouping.len() - 1 {
                let last_group: TextGroup = line.pop().unwrap();
                merge_group.value = format!("{} {}", last_group.value, merge_group.value);
                merge_group.len += last_group.len + 1;
                merge_group.post_whitespace_position = last_group.post_whitespace_position;
            }
            line.push(merge_group);
            break;
        }
    }
}

// block openers and the tokens that close them
const REQUIRED_TO_DECREASE: [(&str, &[&str]); 7] = [
    ("(", &[")"]),
    ("OPEN", &["CLOSE"]),
    ("BEGIN", &["END", "COMMIT", "ROLLBACK"]),
    ("DO", &["END"]),
    ("CASE", &["END"]),
    ("THEN", &["END", "ELSE"]),
    ("ELSE", &["END", "BEGIN"]),
];

#[derive(PartialEq, Eq, Debug)]
enum ParenCategory {
    Space0Newline0,
    Space0Newline1,
    Space1Newline1,
}

#[cfg(test)]
mod tests;
