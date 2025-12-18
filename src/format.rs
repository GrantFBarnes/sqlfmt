use std::collections::HashMap;

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
        state.add_pre_space(input_token, prev_input_token, config);
        state.set_previous_comment_pre_space(config);
        state.push(input_token.clone());
        state.increase_indent_stack(input_token);
        state.decrease_paren_stack(input_token);
        state.align_text_groups(config, input_token);
    }

    return state.get_result(config);
}

struct FormatState {
    tokens: Vec<Token>,
    prefix: Option<String>,
    indent_stack: Vec<Token>,
    paren_stack: Vec<ParenCategory>,
}

impl FormatState {
    fn new() -> FormatState {
        FormatState {
            tokens: vec![],
            prefix: None,
            indent_stack: vec![],
            paren_stack: vec![],
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
            Some(TokenCategory::NewLine) => {
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
                return true;
            }
            Some(TokenCategory::WhiteSpace) => {
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
                    && prev_input_token.is_some_and(|t| t.category == Some(TokenCategory::NewLine))
                {
                    let prev_token: Option<&Token> = self.tokens.last();
                    if prev_token.is_none_or(|t| t.category != Some(TokenCategory::NewLine)) {
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
                return true;
            }
            _ => {
                if self.prefix.is_none() {
                    self.prefix = Some(String::new());
                }
                return false;
            }
        }
    }

    fn increase_paren_stack(&mut self, token: &Token) {
        if token.category != Some(TokenCategory::ParenOpen) {
            return;
        }

        if let Some(prev_token) = self.tokens.last() {
            match prev_token.category {
                Some(TokenCategory::XmlMethod) => {
                    self.paren_stack.push(ParenCategory::Space0Newline0);
                    return;
                }
                Some(TokenCategory::DataType) | Some(TokenCategory::Method) | None => {
                    self.paren_stack.push(ParenCategory::Space0Newline1);
                    return;
                }
                _ => (),
            }
        }

        self.paren_stack.push(ParenCategory::Space1Newline1);
    }

    fn decrease_paren_stack(&mut self, token: &Token) {
        if token.category == Some(TokenCategory::ParenClose) {
            self.paren_stack.pop();
        }
    }

    fn align_text_groups(&mut self, config: &Configuration, token: &Token) {
        if !config.align_text_groups {
            return;
        }

        if token.category != Some(TokenCategory::ParenClose) {
            return;
        }

        struct TextGroup {
            len: usize,
            post_whitespace_position: Option<usize>,
        }

        let mut paren_count: usize = 1;
        let mut indent_len: Option<usize> = None;
        let mut last_whitespace_position: Option<usize> = None;
        let mut current_group: Option<TextGroup> = None;
        let mut current_line: Vec<TextGroup> = vec![];
        let mut lines: Vec<Vec<TextGroup>> = vec![];
        let mut max_groups_per_line: usize = 0;

        // loop backwards until previous newline outside paren set
        for i in (0..self.tokens.len() - 1).rev() {
            let prev_token: &Token = &self.tokens[i];

            if paren_count == 0 {
                break;
            }

            match prev_token.category {
                Some(TokenCategory::WhiteSpace) => {
                    if current_group.is_some() {
                        current_line.push(current_group.unwrap());
                        if current_line.len() > max_groups_per_line {
                            max_groups_per_line = current_line.len();
                        }
                        current_group = None;
                    }
                    last_whitespace_position = Some(i);
                }
                Some(TokenCategory::NewLine) => {
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
                                return;
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
                }
                _ => {
                    match prev_token.category {
                        Some(TokenCategory::ParenOpen) => paren_count -= 1,
                        Some(TokenCategory::ParenClose) => paren_count += 1,
                        _ => (),
                    }
                    if current_group.is_none() {
                        current_group = Some(TextGroup {
                            len: 0,
                            post_whitespace_position: last_whitespace_position,
                        });
                    }
                    let mut new_current_group: TextGroup = current_group.unwrap();
                    new_current_group.len += prev_token.len();
                    current_group = Some(new_current_group);
                }
            }
        }

        // find max len in each group
        let mut max_len_in_each_group: Vec<usize> = vec![];
        for _ in 0..max_groups_per_line {
            max_len_in_each_group.push(0);
        }
        for line in &lines {
            for group_idx in 0..line.len() {
                if line[group_idx].len > max_len_in_each_group[group_idx] {
                    max_len_in_each_group[group_idx] = line[group_idx].len;
                }
            }
        }

        // adjust post whitespace len to align each group
        for line in &lines {
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
        config: &Configuration,
    ) {
        if self
            .tokens
            .last()
            .is_none_or(|t| t.category == Some(TokenCategory::WhiteSpace))
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
                t.category != Some(TokenCategory::WhiteSpace)
                    && t.category != Some(TokenCategory::NewLine)
            })
        {
            return;
        }

        if config.newlines {
            self.insert_newline(token, config);
            self.add_pre_newline(token);
            self.remove_extra_newline(token, config);
        }

        let prev_token: &Token = self
            .tokens
            .last()
            .expect("should always have a previous token");

        if token
            .behavior
            .contains(&TokenBehavior::NoSpaceBeforeIfStartOfNewLine)
            && prev_token.category == Some(TokenCategory::NewLine)
        {
            return;
        }

        match token.category {
            Some(TokenCategory::Delimiter) => {
                if prev_token.value.to_uppercase() != "DELIMITER" {
                    return;
                }
            }
            Some(TokenCategory::ParenOpen) => {
                if let Some(paren) = self.paren_stack.last() {
                    if paren == &ParenCategory::Space0Newline0
                        || paren == &ParenCategory::Space0Newline1
                    {
                        return;
                    }
                }
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
                t.category != Some(TokenCategory::WhiteSpace)
                    && t.category != Some(TokenCategory::NewLine)
            })
        {
            return;
        }

        if prev_token.category == Some(TokenCategory::NewLine) {
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
        if self.get_current_line_length(token) > config.chars.into() {
            self.insert_newline_after_last_operator(self.tokens.len(), config);
        }
    }

    fn get_current_line_length(&self, token: &Token) -> usize {
        let mut line_length: usize = token.len();
        for i in (0..self.tokens.len()).rev() {
            let prev_token: &Token = &self.tokens[i];
            match prev_token.category {
                Some(TokenCategory::NewLine) => break,
                _ => line_length += prev_token.len(),
            }
        }
        return line_length;
    }

    fn add_pre_newline(&mut self, token: &Token) {
        if self.tokens.is_empty() {
            return;
        }

        if self.paren_stack.contains(&ParenCategory::Space0Newline0) {
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

        if prev1_token.behavior.contains(&TokenBehavior::NewLineAfter) {
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

        if prev1_token.category == Some(TokenCategory::ParenOpen) {
            if token.category != Some(TokenCategory::ParenClose) {
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
            .contains(&TokenBehavior::NewLineBeforeIfNotAfterKeyword)
            && prev1_token.category != Some(TokenCategory::Keyword)
        {
            self.push(Token::new_newline());
            return;
        }

        if token
            .behavior
            .contains(&TokenBehavior::NewLineBeforeIfNotAfterEvent)
            && prev1_token.category != Some(TokenCategory::DataType)
            && prev1_token.category != Some(TokenCategory::Event)
            && prev3_token.is_none_or(|t| t.category != Some(TokenCategory::Event))
        {
            self.push(Token::new_newline());
            return;
        }

        if prev1_token
            .behavior
            .contains(&TokenBehavior::NewLineAfterIfNotAfterKeyword)
            && let Some(p3t) = prev3_token
            && p3t.category != Some(TokenCategory::Keyword)
        {
            self.push(Token::new_newline());
            return;
        }

        if prev3_token.is_some_and(|t| t.behavior.contains(&TokenBehavior::NewLineAfterSkip))
            || (prev1_token.category == Some(TokenCategory::ParenClose)
                && prev3_token.is_some_and(|t| t.category == Some(TokenCategory::ParenOpen))
                && self
                    .tokens
                    .iter()
                    .nth_back(4)
                    .is_some_and(|t| t.behavior.contains(&TokenBehavior::NewLineAfterSkip)))
        {
            self.push(Token::new_newline());
            return;
        }
    }

    fn remove_extra_newline(&mut self, token: &Token, config: &Configuration) {
        // collapse paren if short enough
        if token.category == Some(TokenCategory::ParenClose) {
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
                        Some(TokenCategory::NewLine) => break,
                        Some(TokenCategory::Operator) => {
                            if last_operator_len_after.is_none() {
                                last_operator_len_after = Some(collapsed_line_len);
                                last_operator_position = Some(i);
                            }
                        }
                        _ => (),
                    }
                } else {
                    // still inside paren set in question
                    match prev_token.category {
                        Some(TokenCategory::ParenOpen) => paren_count -= 1,
                        Some(TokenCategory::ParenClose) => paren_count += 1,
                        Some(TokenCategory::WhiteSpace) => {
                            if let Some(pt) = self.tokens.get(i - 1)
                                && (pt.category == Some(TokenCategory::NewLine)
                                    || pt.category == Some(TokenCategory::WhiteSpace))
                            {
                                positions_to_remove.push(i);
                                continue;
                            }
                        }
                        Some(TokenCategory::NewLine) => {
                            positions_to_remove.push(i);

                            if let Some(pnwt) = self.get_prev_nonwhitespace_token(i)
                                && pnwt.category != Some(TokenCategory::ParenOpen)
                                && let Some(nnwt) = self.get_next_nonwhitespace_token(i)
                                && nnwt.category != Some(TokenCategory::ParenClose)
                            {
                                positions_to_add_space.push(i);

                                // increase length for space that would replace the newline if collapsed
                                collapsed_line_len += 1;
                                collapsed_inner_len += 1;
                            }
                            continue;
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
                self.insert_newline_after_last_operator(
                    last_operator_position.unwrap() + 1,
                    config,
                );
            }
        }

        // remove double newline
        if token.behavior.contains(&TokenBehavior::NoNewLineBeforeX2) {
            if self.tokens.len() < 2 {
                return;
            }

            if self.tokens[self.tokens.len() - 1].category != Some(TokenCategory::NewLine) {
                return;
            }

            if self.tokens[self.tokens.len() - 2].category != Some(TokenCategory::NewLine) {
                return;
            }

            self.tokens.pop();
            return;
        }

        // remove double newline for two consecutive single delimiter lines
        if token.category == Some(TokenCategory::Delimiter) {
            let mut prev_newline_positions: Vec<usize> = vec![];
            let mut prev_endline_tokens: Vec<&Token> = vec![];
            for i in (1..self.tokens.len()).rev() {
                if self.tokens[i].category == Some(TokenCategory::NewLine) {
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
            if prev_endline_tokens[1].category != Some(TokenCategory::Delimiter) {
                return;
            }

            if prev_endline_tokens.len() == 2
                || prev_endline_tokens[2]
                    .behavior
                    .contains(&TokenBehavior::NoNewLineAfterX2Skip)
            {
                self.tokens.remove(prev_newline_positions[0]);
                return;
            }
        }
    }

    fn get_prev_nonwhitespace_token(&self, pos: usize) -> Option<&Token> {
        for i in (0..std::cmp::min(pos, self.tokens.len())).rev() {
            let prev_token: &Token = &self.tokens[i];
            match prev_token.category {
                Some(TokenCategory::WhiteSpace) => continue,
                Some(TokenCategory::NewLine) => continue,
                _ => return Some(prev_token),
            }
        }
        None
    }

    fn get_next_nonwhitespace_token(&self, pos: usize) -> Option<&Token> {
        for i in pos + 1..self.tokens.len() {
            let next_token: &Token = &self.tokens[i];
            match next_token.category {
                Some(TokenCategory::WhiteSpace) => continue,
                Some(TokenCategory::NewLine) => continue,
                _ => return Some(next_token),
            }
        }
        None
    }

    fn insert_newline_after_last_operator(&mut self, pos: usize, config: &Configuration) {
        for i in (0..std::cmp::min(pos, self.tokens.len())).rev() {
            match self.tokens[i].category {
                Some(TokenCategory::NewLine) => return,
                Some(TokenCategory::Operator) => {
                    if i == self.tokens.len() - 1 {
                        self.tokens.push(Token::new_newline());
                        return;
                    }

                    if self.tokens[i].value == "*"
                        && self.tokens[i + 1].category == Some(TokenCategory::ParenClose)
                    {
                        return;
                    }

                    if self.tokens[i + 1].category == Some(TokenCategory::WhiteSpace) {
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
        return size;
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

        return result;
    }

    fn increase_indent_stack(&mut self, token: &Token) {
        if token.behavior.contains(&TokenBehavior::IncreaseIndent) {
            self.indent_stack.push(token.clone());
            return;
        }

        if token
            .behavior
            .contains(&TokenBehavior::IncreaseIndentIfNotAfterKeyword)
        {
            if self
                .tokens
                .iter()
                .nth_back(2)
                .is_none_or(|t| t.category != Some(TokenCategory::Keyword))
            {
                self.indent_stack.push(token.clone());
                return;
            }
        }

        if token
            .behavior
            .contains(&TokenBehavior::IncreaseIndentIfStandAlone)
        {
            if self.indent_stack.last().is_none_or(|t| {
                !t.behavior
                    .contains(&TokenBehavior::IncreaseIndentIfStandAlone)
            }) {
                self.indent_stack.push(token.clone());
                return;
            }
        }
    }

    fn decrease_indent_stack(&mut self, token: &Token) {
        if self.indent_stack.is_empty() {
            return;
        }

        let required_to_decrease: HashMap<&str, &str> = HashMap::from([
            ("(", ")"),
            ("OPEN", "CLOSE"),
            ("BEGIN", "END"),
            ("DO", "END"),
            ("CASE", "END"),
            ("THEN", "END"),
        ]);

        let mut decrease_until: Vec<&str> = vec![];
        for kv in &required_to_decrease {
            if kv.1 == &token.value.to_uppercase() {
                decrease_until.push(kv.0);
            }
        }

        if !decrease_until.is_empty() {
            loop {
                let top: Option<Token> = self.indent_stack.pop();
                if top.is_none() {
                    return;
                }
                let top: Token = top.unwrap();
                let top_value: String = top.value.to_uppercase();

                if decrease_until.contains(&top_value.as_str()) {
                    return;
                }

                if required_to_decrease.get(top_value.as_str()).is_some() {
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

                if required_to_decrease
                    .get(top.value.to_uppercase().as_str())
                    .is_some()
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

        if self.tokens[token_count - 1].category != Some(TokenCategory::WhiteSpace)
            || self.tokens[token_count - 2].category != Some(TokenCategory::WhiteSpace)
            || self.tokens[token_count - 3].category != Some(TokenCategory::NewLine)
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

        if self.tokens[newline_index - 1].category != Some(TokenCategory::Comment)
            || self.tokens[newline_index - 2].category != Some(TokenCategory::WhiteSpace)
            || self.tokens[newline_index - 3].category != Some(TokenCategory::WhiteSpace)
            || self.tokens[newline_index - 4].category != Some(TokenCategory::NewLine)
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
                Some(TokenCategory::Keyword)
                | Some(TokenCategory::DataType)
                | Some(TokenCategory::Event)
                | Some(TokenCategory::Method) => match config.case {
                    ConfigCase::Uppercase => token_value = token_value.to_uppercase(),
                    ConfigCase::Lowercase => token_value = token_value.to_lowercase(),
                    ConfigCase::Unchanged => (),
                },
                Some(TokenCategory::XmlMethod) => {
                    if result.ends_with(FULL_STOP)
                        && self
                            .tokens
                            .get(i + 1)
                            .is_some_and(|t| t.category == Some(TokenCategory::ParenOpen))
                    {
                        token_value = token_value.to_lowercase();
                    }
                }
                Some(TokenCategory::NewLine) => {
                    result = result
                        .trim_end_matches(|c: char| c.is_whitespace() && c != NEW_LINE)
                        .to_string();
                }
                _ => (),
            }

            result.push_str(token_value.as_str());
        }
        return result
            .trim_end_matches(|c: char| c.is_whitespace() && c != NEW_LINE)
            .to_string();
    }
}

#[derive(PartialEq, Eq, Debug)]
enum ParenCategory {
    Space0Newline0,
    Space0Newline1,
    Space1Newline1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_formatted_sql_empty() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#""#);

        assert_eq!(get_formatted_sql(&config, sql.clone()), r#""#);

        config.newlines = true;
        assert_eq!(get_formatted_sql(&config, sql.clone()), r#""#);
    }

    #[test]
    fn test_get_formatted_sql_whitespace_only() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"


        "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"


"#,
        );

        config.newlines = true;
        assert_eq!(get_formatted_sql(&config, sql.clone()), r#""#);
    }

    #[test]
    fn test_get_formatted_sql_select_simple() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"SELECT * FROM TBL1"#);

        config.newlines = false;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SELECT * FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_newlines() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"SELECT * FROM TBL1"#);

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SELECT
    *
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_upper() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"select * from tbl1"#);

        config.case = ConfigCase::Uppercase;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SELECT * FROM tbl1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_lower() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"SELECT * FROM TBL1"#);

        config.case = ConfigCase::Lowercase;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"select * from TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_simple_newlines() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT  *
            FROM  TBL1
            "#,
        );

        config.newlines = false;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT *
            FROM TBL1
"#
        );
    }

    #[test]
    fn test_get_formatted_sql_config_tabs() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
                C1,
                C2
            FROM TBL1
            "#,
        );

        config.tabs = ConfigTab::Tab;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT
            	C1,
            	C2
            FROM TBL1
"#
        );
    }

    #[test]
    fn test_get_formatted_sql_prefix_tabs() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
			SELECT
				C1,
				C2
			FROM TBL1
            "#,
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"			SELECT
			    C1,
			    C2
			FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_config_spaces() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
                C1,
                C2
            FROM TBL1
            "#,
        );

        config.tabs = ConfigTab::Space(2);
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT
              C1,
              C2
            FROM TBL1
"#
        );
    }

    #[test]
    fn test_get_formatted_sql_config_chars() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
            (SELECT C1, C2 FROM TBL1)
            "#,
        );

        config.newlines = true;

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                (SELECT C1, C2 FROM TBL1)"#
        );

        config.chars = 40;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                (
                    SELECT
                        C1,
                        C2
                    FROM TBL1
                )"#
        );
    }

    #[test]
    fn test_get_formatted_sql_config_chars_tabs() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
			SELECT
			(SELECT C1, C2 FROM TBL1)
			"#,
        );

        config.tabs = ConfigTab::Tab;
        config.newlines = true;

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"			SELECT
				(SELECT C1, C2 FROM TBL1)"#
        );

        config.chars = 40;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"			SELECT
				(
					SELECT
						C1,
						C2
					FROM TBL1
				)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_config_chars_operators() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
            COLUMN1 + COLUMN2 + COLUMN3 + COLUMN4 + COLUMN5 + COLUMN6 + COLUMN7
            FROM TBL1
            "#,
        );

        config.newlines = true;

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                COLUMN1 + COLUMN2 + COLUMN3 + COLUMN4 + COLUMN5 + COLUMN6 +
                COLUMN7
            FROM TBL1"#
        );

        config.chars = 40;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                COLUMN1 + COLUMN2 +
                COLUMN3 + COLUMN4 +
                COLUMN5 + COLUMN6 +
                COLUMN7
            FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_config_chars_paren_operators() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
            (SELECT COLUMN1, COLUMN2 FROM TBL1) + (SELECT COLUMN1, COLUMN2 FROM TBL1)
            "#,
        );

        config.newlines = true;

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                (SELECT COLUMN1, COLUMN2 FROM TBL1) +
                (SELECT COLUMN1, COLUMN2 FROM TBL1)"#
        );

        config.chars = 40;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                (
                    SELECT
                        COLUMN1,
                        COLUMN2
                    FROM TBL1
                ) +
                (
                    SELECT
                        COLUMN1,
                        COLUMN2
                    FROM TBL1
                )"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_multiple_columns_inline() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"SELECT C1,C2, C3 FROM TBL1"#);

        config.newlines = false;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SELECT C1, C2, C3 FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_multiple_columns_newlines() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
            C1 AS 'Column 1',
                C2 AS 'Column 2',
            C3
            FROM TBL1 AS T
            "#,
        );

        config.newlines = false;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT
                C1 AS 'Column 1',
                C2 AS 'Column 2',
                C3
            FROM TBL1 AS T
"#
        );
    }

    #[test]
    fn test_get_formatted_sql_alias() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"SELECT C1 AS 'Column 1' FROM TBL1"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SELECT C1 AS 'Column 1' FROM TBL1"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SELECT
    C1 AS 'Column 1'
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_go() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT C1 FROM TBL1 GO
            SELECT C1 FROM TBL1 GO
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT C1 FROM TBL1 GO
            SELECT C1 FROM TBL1 GO
"#,
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1
            FROM TBL1
            GO

            SELECT
                C1
            FROM TBL1
            GO"#,
        );
    }

    #[test]
    fn test_get_formatted_sql_datatype_quote() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"[NVARCHAR](36)"#);

        assert_eq!(get_formatted_sql(&config, sql.clone()), r#"[NVARCHAR](36)"#);

        config.newlines = true;
        assert_eq!(get_formatted_sql(&config, sql.clone()), r#"[NVARCHAR](36)"#);
    }

    #[test]
    fn test_get_formatted_sql_convert() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"CONVERT(NVARCHAR(36), ID)"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"CONVERT(NVARCHAR(36), ID)"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"CONVERT(NVARCHAR(36), ID)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_curly_string() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT T1.*,  {columnName1},{columnName2} , {columnName3}
            FROM {tableNames[i]} AS T1
            INNER JOIN   {tableNames[i]}   AS T2 ON T2.C1 = T1.C1
            INNER JOIN T{tableNames[i]}3 AS T3 ON T3.C1 = T1.C1
            INNER JOIN   T{tableNames[i]}4   AS T4 ON T4.C1 = T1.C1
            {otherJoin}
            WHERE T1.C2 = 1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT T1.*, {columnName1},{columnName2}, {columnName3}
            FROM {tableNames[i]} AS T1
                INNER JOIN {tableNames[i]} AS T2 ON T2.C1 = T1.C1
                INNER JOIN T{tableNames[i]}3 AS T3 ON T3.C1 = T1.C1
                INNER JOIN T{tableNames[i]}4 AS T4 ON T4.C1 = T1.C1
                {otherJoin}
            WHERE T1.C2 = 1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                T1.*,
                {columnName1},{columnName2},
                {columnName3}
            FROM {tableNames[i]} AS T1
                INNER JOIN {tableNames[i]} AS T2 ON T2.C1 = T1.C1
                INNER JOIN T{tableNames[i]}3 AS T3 ON T3.C1 = T1.C1
                INNER JOIN T{tableNames[i]}4 AS T4 ON T4.C1 = T1.C1 {otherJoin}
            WHERE T1.C2 = 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_call_curly_string() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            CALL SCH.{procedureName}();
            CALL SCH.B{procedureName}E();
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            CALL SCH.{procedureName}();
            CALL SCH.B{procedureName}E();
"#,
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            CALL SCH.{procedureName}();
            CALL SCH.B{procedureName}E();"#,
        );
    }

    #[test]
    fn test_get_formatted_sql_embedded_conditions() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT * FROM TBL1
            WHERE ((C1=0 AND C2=0)OR(C1=1 AND C2=1))
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT * FROM TBL1
            WHERE ((C1 = 0 AND C2 = 0) OR (C1 = 1 AND C2 = 1))
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                *
            FROM TBL1
            WHERE ((C1 = 0 AND C2 = 0) OR (C1 = 1 AND C2 = 1))"#
        );
    }

    #[test]
    fn test_get_formatted_sql_top() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT TOP 100 ID FROM TBL1;
            SELECT TOP (100) ID FROM TBL1;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT TOP 100 ID FROM TBL1;
            SELECT TOP (100) ID FROM TBL1;
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT TOP 100
                ID
            FROM TBL1;

            SELECT TOP (100)
                ID
            FROM TBL1;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_sub_query_inline() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"SELECT ( SELECT TOP 1 ID FROM TBL1 ) AS ID"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SELECT (SELECT TOP 1 ID FROM TBL1) AS ID"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SELECT
    (SELECT TOP 1 ID FROM TBL1) AS ID"#
        );
    }

    #[test]
    fn test_get_formatted_sql_union() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"SELECT C1 UNION SELECT C2"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SELECT C1 UNION SELECT C2"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SELECT
    C1
UNION
SELECT
    C2"#
        );
    }

    #[test]
    fn test_get_formatted_sql_union_complex() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT C1 FROM TBL1
            UNION SELECT C2 FROM TBL2
            UNION SELECT C3 FROM TBL3
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT C1 FROM TBL1
            UNION SELECT C2 FROM TBL2
            UNION SELECT C3 FROM TBL3
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1
            FROM TBL1
            UNION
            SELECT
                C2
            FROM TBL2
            UNION
            SELECT
                C3
            FROM TBL3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_comma_start_multiline() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
            C1
            ,C2
            ,C3
            FROM TBL1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT
                C1
                , C2
                , C3
            FROM TBL1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1,
                C2,
                C3
            FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_sub_query_multiline() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT (
            SELECT TOP 1 ID FROM TBL1
            ) AS ID,
            C1
            FROM TBL1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT (
                    SELECT TOP 1 ID FROM TBL1
                ) AS ID,
                C1
            FROM TBL1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                (SELECT TOP 1 ID FROM TBL1) AS ID,
                C1
            FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT * FROM TBL1;DECLARE C1=1;DECLARE C2= 2;  DECLARE C3 = 3;SELECT * FROM TBL1  DECLARE C4=4;DECLARE C5=5;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT * FROM TBL1; DECLARE C1 = 1; DECLARE C2 = 2; DECLARE C3 = 3; SELECT * FROM TBL1 DECLARE C4 = 4; DECLARE C5 = 5;
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                *
            FROM TBL1;

            DECLARE C1 = 1;
            DECLARE C2 = 2;
            DECLARE C3 = 3;

            SELECT
                *
            FROM TBL1
            DECLARE C4 = 4;

            DECLARE C5 = 5;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_comments_only() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
-- COMMENT
  -- COMMENT
    -- COMMENT
  -- COMMENT
-- COMMENT
-- COMMENT
/*COMMENT*//*COMMENT*/ /*COMMENT*/
    /*COMMENT*/ /*COMMENT*/
    /*COMMENT*/ /*COMMENT*/
                    "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
-- COMMENT
  -- COMMENT
    -- COMMENT
  -- COMMENT
-- COMMENT
-- COMMENT
/*COMMENT*//*COMMENT*/ /*COMMENT*/
    /*COMMENT*/ /*COMMENT*/
    /*COMMENT*/ /*COMMENT*/
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"-- COMMENT
  -- COMMENT
    -- COMMENT
  -- COMMENT
-- COMMENT
-- COMMENT
/*COMMENT*//*COMMENT*/
/*COMMENT*/
    /*COMMENT*/
/*COMMENT*/
    /*COMMENT*/
/*COMMENT*/"#
        );

        config.comment_pre_space = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"-- COMMENT
-- COMMENT
-- COMMENT
-- COMMENT
-- COMMENT
-- COMMENT
/*COMMENT*//*COMMENT*/
/*COMMENT*/
/*COMMENT*/
/*COMMENT*/
/*COMMENT*/
/*COMMENT*/"#
        );
    }

    #[test]
    fn test_get_formatted_sql_comments_with_statements() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
    -- COMMENT
        SELECT 1;
    -- COMMENT
    SELECT 1;
-- COMMENT
SELECT 1;
    -- COMMENT
    -- COMMENT
    SELECT 1;
        -- COMMENT
        SELECT 1;
        -- COMMENT
        SELECT 1
    -- COMMENT
    SELECT 1
-- COMMENT
SELECT 1
    -- COMMENT
    SELECT 1
        -- COMMENT
        SELECT 1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
    -- COMMENT
    SELECT 1;
    -- COMMENT
    SELECT 1;
-- COMMENT
    SELECT 1;
    -- COMMENT
    -- COMMENT
    SELECT 1;
        -- COMMENT
    SELECT 1;
        -- COMMENT
    SELECT 1
    -- COMMENT
    SELECT 1
-- COMMENT
    SELECT 1
    -- COMMENT
    SELECT 1
        -- COMMENT
    SELECT 1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"    -- COMMENT
    SELECT
        1;

    -- COMMENT
    SELECT
        1;

-- COMMENT
    SELECT
        1;

    -- COMMENT
    -- COMMENT
    SELECT
        1;

        -- COMMENT
    SELECT
        1;

        -- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1
-- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1
        -- COMMENT
    SELECT
        1"#
        );

        config.comment_pre_space = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"    -- COMMENT
    SELECT
        1;

    -- COMMENT
    SELECT
        1;

    -- COMMENT
    SELECT
        1;

    -- COMMENT
    -- COMMENT
    SELECT
        1;

    -- COMMENT
    SELECT
        1;

    -- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter_comment() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            -- COMMENT
            DECLARE C1=1;DECLARE C2=2;
            -- COMMENT
            DECLARE C1=1;DECLARE C2=2;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            -- COMMENT
            DECLARE C1 = 1; DECLARE C2 = 2;
            -- COMMENT
            DECLARE C1 = 1; DECLARE C2 = 2;
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            -- COMMENT
            DECLARE C1 = 1;
            DECLARE C2 = 2;

            -- COMMENT
            DECLARE C1 = 1;
            DECLARE C2 = 2;"#
        );

        config.comment_pre_space = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            -- COMMENT
            DECLARE C1 = 1;
            DECLARE C2 = 2;

            -- COMMENT
            DECLARE C1 = 1;
            DECLARE C2 = 2;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter_change() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT 1;DELIMITER $$ SELECT 1; DELIMITER ;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT 1; DELIMITER $$ SELECT 1; DELIMITER ;
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                1;

            DELIMITER $$

            SELECT
                1;
            DELIMITER ;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_declare_no_delimiter() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"DECLARE C1 = 1 DECLARE C2 = 2   DECLARE C3 = 3 "#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"DECLARE C1 = 1 DECLARE C2 = 2 DECLARE C3 = 3"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"DECLARE C1 = 1
DECLARE C2 = 2
DECLARE C3 = 3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_multiple_declare() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"DECLARE C1 = 1, C2 = 2, C3 = 3;"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"DECLARE C1 = 1, C2 = 2, C3 = 3;"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"DECLARE C1 = 1,
    C2 = 2,
    C3 = 3;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_set_no_delimiter() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"SET C1 = 1 SET C2 = 2   SET C3 = 3 "#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SET C1 = 1 SET C2 = 2 SET C3 = 3"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"SET C1 = 1
SET C2 = 2
SET C3 = 3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_set() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SET C1 = 1
            SET C2 = 2
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SET C1 = 1
            SET C2 = 2
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SET C1 = 1
            SET C2 = 2"#
        );
    }

    #[test]
    fn test_get_formatted_sql_update() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            UPDATE TBL1
            SET
            C1 = 1,
            C2 = 2
            WHERE C3 = 3
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            UPDATE TBL1
            SET
                C1 = 1,
                C2 = 2
            WHERE C3 = 3
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            UPDATE TBL1
            SET C1 = 1,
                C2 = 2
            WHERE C3 = 3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where_quote() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT *
            FROM TBL1
            WHERE C1 = 'some value'
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT *
            FROM TBL1
            WHERE C1 = 'some value'
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                *
            FROM TBL1
            WHERE C1 = 'some value'"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where_in() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT *
            FROM TBL1
            WHERE C1 IN ('VALUE 1','VALUE 2','VALUE 3','VALUE 4','VALUE 5','VALUE 6');
            SELECT *
            FROM TBL1
            WHERE C1 IN (
                'VALUE 1',
                'VALUE 2',
                'VALUE 3',
                'VALUE 4',
                'VALUE 5',
                'VALUE 6'
            );
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT *
            FROM TBL1
            WHERE C1 IN ('VALUE 1', 'VALUE 2', 'VALUE 3', 'VALUE 4', 'VALUE 5', 'VALUE 6');
            SELECT *
            FROM TBL1
            WHERE C1 IN (
                    'VALUE 1',
                    'VALUE 2',
                    'VALUE 3',
                    'VALUE 4',
                    'VALUE 5',
                    'VALUE 6'
                );
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                *
            FROM TBL1
            WHERE C1 IN (
                    'VALUE 1',
                    'VALUE 2',
                    'VALUE 3',
                    'VALUE 4',
                    'VALUE 5',
                    'VALUE 6'
                );

            SELECT
                *
            FROM TBL1
            WHERE C1 IN (
                    'VALUE 1',
                    'VALUE 2',
                    'VALUE 3',
                    'VALUE 4',
                    'VALUE 5',
                    'VALUE 6'
                );"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where_in_cr() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT *
            FROM TBL1
            WHERE C1 IN (
                'VALUE 1',
                'VALUE 2',
                'VALUE 3',
                'VALUE 4',
                'VALUE 5',
                'VALUE 6'
            );
            "#,
        );
        let sql: String = sql.replace('\n', "\r\n");

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT *
            FROM TBL1
            WHERE C1 IN (
                    'VALUE 1',
                    'VALUE 2',
                    'VALUE 3',
                    'VALUE 4',
                    'VALUE 5',
                    'VALUE 6'
                );
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                *
            FROM TBL1
            WHERE C1 IN (
                    'VALUE 1',
                    'VALUE 2',
                    'VALUE 3',
                    'VALUE 4',
                    'VALUE 5',
                    'VALUE 6'
                );"#
        );
    }

    #[test]
    fn test_get_formatted_sql_count_distinct() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT COUNT(DISTINCT YEAR(D1))
            FROM TBL1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT COUNT(DISTINCT YEAR(D1))
            FROM TBL1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                COUNT(DISTINCT YEAR(D1))
            FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_paren_count() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            (SELECT COUNT(*) FROM TBL1)
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            (SELECT COUNT(*) FROM TBL1)
"#
        );

        config.newlines = true;
        config.chars = 10;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            (
                SELECT
                    COUNT(*)
                FROM TBL1
            )"#
        );

        config.align_text_groups = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            (
                SELECT
                    COUNT(*)
                FROM TBL1
            )"#
        );
    }

    #[test]
    fn test_get_formatted_sql_paren_collapse() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            BEGIN
            SELECT
            REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_NO_PARAMETER(),
            REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_ONE_PARAMETER(P1),
            REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_TWO_PARAMETER(P1,P2),
            ROUND((LENGTH(LONG_VARIABLE_NAME) - LENGTH(REPLACE(LONG_VARIABLE_NAME, '_____', ''))) / LENGTH('_____')) AS BLANKCOUNT
            END
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            BEGIN
                SELECT
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_NO_PARAMETER(),
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_ONE_PARAMETER(P1),
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_TWO_PARAMETER(P1, P2),
                    ROUND((LENGTH(LONG_VARIABLE_NAME) - LENGTH(REPLACE(LONG_VARIABLE_NAME, '_____', ''))) / LENGTH('_____')) AS BLANKCOUNT
            END
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            BEGIN
                SELECT
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_NO_PARAMETER(),
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_ONE_PARAMETER(P1),
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_TWO_PARAMETER(
                        P1,
                        P2
                    ),
                    ROUND(
                        (
                            LENGTH(LONG_VARIABLE_NAME) -
                            LENGTH(REPLACE(LONG_VARIABLE_NAME, '_____', ''))
                        ) / LENGTH('_____')
                    ) AS BLANKCOUNT
            END"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_group_by() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT C1,
            COUNT(*) AS CNT
            FROM TBL1
            GROUP BY C1
            HAVING COUNT(*) > 1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT C1,
                COUNT(*) AS CNT
            FROM TBL1
            GROUP BY C1
            HAVING COUNT(*) > 1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1,
                COUNT(*) AS CNT
            FROM TBL1
            GROUP BY C1
            HAVING COUNT(*) > 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_join() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT T1.C1, T1.C2,
            T2.C2
            FROM TBL1 AS T1
            INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT T1.C1, T1.C2,
                T2.C2
            FROM TBL1 AS T1
                INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                T1.C1,
                T1.C2,
                T2.C2
            FROM TBL1 AS T1
                INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
            C1,
            C2,
            C3
            FROM TBL1
            WHERE C1>1
            AND C2 IS NOT NULL
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT
                C1,
                C2,
                C3
            FROM TBL1
            WHERE C1 > 1
                AND C2 IS NOT NULL
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1,
                C2,
                C3
            FROM TBL1
            WHERE C1 > 1
                AND C2 IS NOT NULL"#
        );
    }

    #[test]
    fn test_get_formatted_sql_multi_join() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT DISTINCT
            T1.C1 AS C1,
            T2.C2 AS C2,
            T3.C3 AS C3
            FROM TBL1 AS T1
            INNER JOIN TBL2 AS T2
            ON T2.C1 = T1.C1
            AND T2.C2 = T1.C2
            INNER JOIN TBL3 AS T3 ON T3.C2 = T2.C2
            WHERE (T1.C2<>T2.C2 OR T1.C2<>T3.C2)
            ORDER BY T1.C1
            LIMIT 1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT DISTINCT
                T1.C1 AS C1,
                T2.C2 AS C2,
                T3.C3 AS C3
            FROM TBL1 AS T1
                INNER JOIN TBL2 AS T2
                ON T2.C1 = T1.C1
                AND T2.C2 = T1.C2
                INNER JOIN TBL3 AS T3 ON T3.C2 = T2.C2
            WHERE (T1.C2 <> T2.C2 OR T1.C2 <> T3.C2)
            ORDER BY T1.C1
            LIMIT 1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT DISTINCT
                T1.C1 AS C1,
                T2.C2 AS C2,
                T3.C3 AS C3
            FROM TBL1 AS T1
                INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
                AND T2.C2 = T1.C2
                INNER JOIN TBL3 AS T3 ON T3.C2 = T2.C2
            WHERE (T1.C2 <> T2.C2 OR T1.C2 <> T3.C2)
            ORDER BY T1.C1
            LIMIT 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_two_statements() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"SELECT * FROM TBL1;SELECT * FROM TBL1;"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            String::from("SELECT * FROM TBL1; SELECT * FROM TBL1;")
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            String::from(
                r#"SELECT
    *
FROM TBL1;

SELECT
    *
FROM TBL1;"#
            )
        );
    }

    #[test]
    fn test_get_formatted_sql_single_comments() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            -- top comment
            SELECT C1,--inline comment
            -- after comment 1
            -- after comment 2
            C2
            -- after comment 3
            FROM TBL1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            -- top comment
            SELECT C1, --inline comment
            -- after comment 1
            -- after comment 2
                C2
            -- after comment 3
            FROM TBL1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            -- top comment
            SELECT
                C1, --inline comment
            -- after comment 1
            -- after comment 2
                C2
            -- after comment 3
            FROM TBL1"#
        );

        config.comment_pre_space = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            -- top comment
            SELECT
                C1,
                --inline comment
                -- after comment 1
                -- after comment 2
                C2
            -- after comment 3
            FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_single_comment_new_statement() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            -- comment
            SELECT
                C1,
                C2
            FROM TBL1;

            -- comment
            SELECT
                C1,
                C2
            FROM TBL1;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            -- comment
            SELECT
                C1,
                C2
            FROM TBL1;

            -- comment
            SELECT
                C1,
                C2
            FROM TBL1;
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            -- comment
            SELECT
                C1,
                C2
            FROM TBL1;

            -- comment
            SELECT
                C1,
                C2
            FROM TBL1;"#
        );

        config.comment_pre_space = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            -- comment
            SELECT
                C1,
                C2
            FROM TBL1;

            -- comment
            SELECT
                C1,
                C2
            FROM TBL1;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_single_comment_before_set() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT C1
            FROM TBL1
            ORDER BY C1

            -- COMMENT
            SET V1 = 1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT C1
            FROM TBL1
            ORDER BY C1

            -- COMMENT
            SET V1 = 1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1
            FROM TBL1
            ORDER BY C1
            -- COMMENT
            SET V1 = 1"#
        );

        config.comment_pre_space = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1
            FROM TBL1
            ORDER BY C1
            -- COMMENT
            SET V1 = 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_multiline_comments() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            /* top comment */
            SELECT C1/* inline comment */
            /*

            after

            comment
                indent

            */FROM TBL1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            /* top comment */
            SELECT C1 /* inline comment */
            /*

            after

            comment
                indent

            */ FROM TBL1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            /* top comment */
            SELECT
                C1 /* inline comment */
            /*

            after

            comment
                indent

            */
            FROM TBL1"#
        );

        config.comment_pre_space = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            /* top comment */
            SELECT
                C1
            /* inline comment */
            /*

            after

            comment
                indent

            */
            FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_into() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
            C1,
            C2,
            C3
            INTO TBL2
            FROM TBL1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT
                C1,
                C2,
                C3
            INTO TBL2
            FROM TBL1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1,
                C2,
                C3
            INTO
                TBL2
            FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_with_nolock() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT C1 FROM TBL1 WITH (NOLOCK)
            SELECT C1 FROM TBL1 WITH (NOLOCK)
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT C1 FROM TBL1 WITH (NOLOCK)
            SELECT C1 FROM TBL1 WITH (NOLOCK)
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1
            FROM TBL1 WITH (NOLOCK)
            SELECT
                C1
            FROM TBL1 WITH (NOLOCK)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_cte_after_select() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT C1 FROM TBL1;
            WITH CTE2 AS
            (SELECT C2 FROM TBL2)
            SELECT * FROM CTE2
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT C1 FROM TBL1;
            WITH CTE2 AS
                (SELECT C2 FROM TBL2)
            SELECT * FROM CTE2
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1
            FROM TBL1;

            WITH CTE2 AS (SELECT C2 FROM TBL2)
            SELECT
                *
            FROM CTE2"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_after_cte() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            WITH CTE1 AS
            (SELECT C1 FROM TBL1)
            INSERT INTO TBL2 (C1)
            SELECT C1 FROM CTE1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            WITH CTE1 AS
                (SELECT C1 FROM TBL1)
            INSERT INTO TBL2(C1)
            SELECT C1 FROM CTE1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            WITH CTE1 AS (SELECT C1 FROM TBL1)
            INSERT INTO TBL2(C1)
            SELECT
                C1
            FROM CTE1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_after_cte_config_newline_long() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            WITH CTE1 AS
            (SELECT C00000000000000000000000000000,C00000000000000000000000000001,C00000000000000000000000000002 FROM TBL1)
            INSERT INTO TBL2 (C00000000000000000000000000000,C00000000000000000000000000001,C00000000000000000000000000002)
            SELECT C00000000000000000000000000000,C00000000000000000000000000001,C00000000000000000000000000002 FROM CTE1
            "#,
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            WITH CTE1 AS (
                    SELECT
                        C00000000000000000000000000000,
                        C00000000000000000000000000001,
                        C00000000000000000000000000002
                    FROM TBL1
                )
            INSERT INTO TBL2(
                C00000000000000000000000000000,
                C00000000000000000000000000001,
                C00000000000000000000000000002
            )
            SELECT
                C00000000000000000000000000000,
                C00000000000000000000000000001,
                C00000000000000000000000000002
            FROM CTE1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_multiple_cte() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            WITH CTE1 AS
            (SELECT C1 FROM TBL1),
            CTE2 AS
            (SELECT C2 FROM TBL2)
            SELECT * FROM CTE1
            INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            WITH CTE1 AS
                (SELECT C1 FROM TBL1),
                CTE2 AS
                (SELECT C2 FROM TBL2)
            SELECT * FROM CTE1
                INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            WITH CTE1 AS (SELECT C1 FROM TBL1),
                CTE2 AS (SELECT C2 FROM TBL2)
            SELECT
                *
            FROM CTE1
                INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_join_subquery() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT * FROM T1
            LEFT JOIN (SELECT C2 FROM T2) AS ST1 ON ST1.C2 = T1.C1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT * FROM T1
                LEFT JOIN (SELECT C2 FROM T2) AS ST1 ON ST1.C2 = T1.C1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                *
            FROM T1
                LEFT JOIN (SELECT C2 FROM T2) AS ST1 ON ST1.C2 = T1.C1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_join_multi_condition() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT * FROM T1
            RIGHT JOIN T2 ON T2.C1 = T1.C1 OR T2.C2 = T1.C2
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT * FROM T1
                RIGHT JOIN T2 ON T2.C1 = T1.C1 OR T2.C2 = T1.C2
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                *
            FROM T1
                RIGHT JOIN T2 ON T2.C1 = T1.C1
                OR T2.C2 = T1.C2"#
        );
    }

    #[test]
    fn test_get_formatted_sql_join_multi_condition_embedded() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT * FROM T1
            FULL OUTER JOIN T2 ON (T2.C1 = T1.C1 OR T2.C2 = T1.C2)
            AND (T2.C3 = T1.C3 OR T2.C4 = T1.C4)
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT * FROM T1
                FULL OUTER JOIN T2 ON (T2.C1 = T1.C1 OR T2.C2 = T1.C2)
                AND (T2.C3 = T1.C3 OR T2.C4 = T1.C4)
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                *
            FROM T1
                FULL OUTER JOIN T2 ON (T2.C1 = T1.C1 OR T2.C2 = T1.C2)
                AND (T2.C3 = T1.C3 OR T2.C4 = T1.C4)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_if() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
                C1,
                IIF(C1 > 5, 1, 0) AS 'IIF',
                C2,
                IF(C2 > 5, 1, 0) AS 'IF'
            FROM TBL1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT
                C1,
                IIF(C1 > 5, 1, 0) AS 'IIF',
                C2,
            IF(C2 > 5, 1, 0) AS 'IF'
            FROM TBL1
"#,
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1,
                IIF(C1 > 5, 1, 0) AS 'IIF',
                C2,
            IF(C2 > 5, 1, 0) AS 'IF'
            FROM TBL1"#,
        );
    }

    #[test]
    fn test_get_formatted_sql_case() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
            C1,
            CASE WHEN C1<=1 THEN 'small'
            WHEN C1<=3 THEN 'medium'
            ELSE 'large' END AS C2,
            C3
            FROM TBL1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT
                C1,
                CASE WHEN C1 <= 1 THEN 'small'
                    WHEN C1 <= 3 THEN 'medium'
                    ELSE 'large' END AS C2,
                C3
            FROM TBL1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1,
                CASE
                    WHEN C1 <= 1 THEN 'small'
                    WHEN C1 <= 3 THEN 'medium'
                    ELSE 'large'
                END AS C2,
                C3
            FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_simple() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"INSERT INTO TBL1(ID)VALUES(1)"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"INSERT INTO TBL1(ID) VALUES (1)"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"INSERT INTO TBL1(ID)
VALUES (1)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_multiple_columns() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            "INSERT INTO TBL1 (C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,C15,C16,C17,C18,C19,C20,C21) VALUES (1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21)",
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"INSERT INTO TBL1(C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21) VALUES (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21)"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"INSERT INTO TBL1(
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
    C20,
    C21
)
VALUES (
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21
)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_select() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            INSERT INTO TBL1 (C1,C2,C3)
            SELECT C1,C2,C3
            FROM TBL1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            INSERT INTO TBL1(C1, C2, C3)
            SELECT C1, C2, C3
            FROM TBL1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            INSERT INTO TBL1(C1, C2, C3)
            SELECT
                C1,
                C2,
                C3
            FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delete_simple() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"DELETE FROM TBL1 WHERE C<=1"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"DELETE FROM TBL1 WHERE C <= 1"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"DELETE
FROM TBL1
WHERE C <= 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delete_newline() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            DELETE
            FROM TBL1
            WHERE C<=1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            DELETE
            FROM TBL1
            WHERE C <= 1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            DELETE
            FROM TBL1
            WHERE C <= 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_truncate_table() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            TRUNCATE TABLE TBL1 TRUNCATE TABLE TBL2
            TRUNCATE TABLE TBL3
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            TRUNCATE TABLE TBL1 TRUNCATE TABLE TBL2
            TRUNCATE TABLE TBL3
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            TRUNCATE TABLE TBL1
            TRUNCATE TABLE TBL2
            TRUNCATE TABLE TBL3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_drop_table() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            DROP TABLE IF EXISTS TBL1 DROP TABLE TBL2
            DROP TABLE TBL3
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            DROP TABLE IF EXISTS TBL1 DROP TABLE TBL2
            DROP TABLE TBL3
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            DROP TABLE IF EXISTS TBL1
            DROP TABLE TBL2
            DROP TABLE TBL3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_execute() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            EXEC SP1();EXEC SP1();
            EXEC SP1();
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            EXEC SP1(); EXEC SP1();
            EXEC SP1();
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            EXEC SP1();
            EXEC SP1();
            EXEC SP1();"#
        );
    }

    #[test]
    fn test_get_formatted_sql_execute_no_delimiter() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"EXEC SP1() EXEC SP1() EXEC SP1()"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"EXEC SP1() EXEC SP1() EXEC SP1()"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"EXEC SP1()
EXEC SP1()
EXEC SP1()"#
        );
    }

    #[test]
    fn test_get_formatted_sql_execute_parameters() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"EXEC SP1 P1, P2, P3 EXEC SP1 P1, P2, P3"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"EXEC SP1 P1, P2, P3 EXEC SP1 P1, P2, P3"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"EXEC SP1 P1,
    P2,
    P3
EXEC SP1 P1,
    P2,
    P3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_call_no_delimiter() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"CALL SP1() CALL SP1() CALL SP1()"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"CALL SP1() CALL SP1() CALL SP1()"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"CALL SP1()
CALL SP1()
CALL SP1()"#
        );
    }

    #[test]
    fn test_get_formatted_sql_if() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            IF V1 IS NULL AND V2 IS NULL BEGIN SET V1 = 0; SET V2 = 0; END
            IF V1 IS NULL THEN SET V1 = 0 END IF
            IF V2 IS NULL SET V2 = 0
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            IF V1 IS NULL AND V2 IS NULL BEGIN SET V1 = 0; SET V2 = 0; END
            IF V1 IS NULL THEN SET V1 = 0 END IF
            IF V2 IS NULL SET V2 = 0
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            IF V1 IS NULL
            AND V2 IS NULL
            BEGIN
                SET V1 = 0;
                SET V2 = 0;
            END
            IF V1 IS NULL THEN
                SET V1 = 0
            END
            IF
            IF V2 IS NULL
            SET V2 = 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_if_else() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            IF V1 IS NULL THEN SET V1 = 0
            ELSE SET V2 = NULL
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            IF V1 IS NULL THEN SET V1 = 0
                ELSE SET V2 = NULL
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            IF V1 IS NULL THEN
                SET V1 = 0
                ELSE
                SET V2 = NULL"#
        );
    }

    #[test]
    fn test_get_formatted_sql_if_else_begin_end() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            IF V1 IS NULL BEGIN
            SET V1 = 0;
            END
            ELSE BEGIN
            SET V2 = NULL;
            END
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            IF V1 IS NULL BEGIN
                SET V1 = 0;
            END
            ELSE BEGIN
                SET V2 = NULL;
            END
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            IF V1 IS NULL
            BEGIN
                SET V1 = 0;
            END
            ELSE
            BEGIN
                SET V2 = NULL;
            END"#
        );
    }

    #[test]
    fn test_get_formatted_sql_try_catch() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SET V1 = 0;
            BEGIN TRY
                CALL SP1;
            END TRY
            BEGIN CATCH
                CALL SP2;
                RETURN 1;
            END CATCH
            RETURN 0
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SET V1 = 0;
            BEGIN TRY
                CALL SP1;
            END TRY
            BEGIN CATCH
                CALL SP2;
                RETURN 1;
            END CATCH
            RETURN 0
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SET V1 = 0;

            BEGIN TRY
                CALL SP1;
            END TRY
            BEGIN CATCH
                CALL SP2;
                RETURN 1;
            END CATCH

            RETURN 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_try_catch_insert() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SET V1 = 0;
            BEGIN TRY
                -- COMMENT
                INSERT INTO TBL1 (C1) VALUES (1)
            END TRY
            BEGIN CATCH
                RETURN 1
            END CATCH
            RETURN 0
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SET V1 = 0;
            BEGIN TRY
                -- COMMENT
                INSERT INTO TBL1(C1) VALUES (1)
            END TRY
            BEGIN CATCH
                RETURN 1
            END CATCH
            RETURN 0
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SET V1 = 0;

            BEGIN TRY
                -- COMMENT
                INSERT INTO TBL1(C1)
                VALUES (1)
            END TRY
            BEGIN CATCH
                RETURN 1
            END CATCH

            RETURN 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_catch_update() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            BEGIN CATCH END CATCH UPDATE TBL1 SET C1 = 1
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            BEGIN CATCH END CATCH UPDATE TBL1 SET C1 = 1
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            BEGIN CATCH
            END CATCH

            UPDATE TBL1
            SET C1 = 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_return() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT *
            FROM TBL
            RETURN 0
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT *
            FROM TBL
            RETURN 0
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                *
            FROM TBL
            RETURN 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_declare_select() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            DECLARE V1 INT = (
            SELECT C1
            FROM TBL
            );
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            DECLARE V1 INT = (
                    SELECT C1
                    FROM TBL
                );
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            DECLARE V1 INT = (SELECT C1 FROM TBL);"#
        );
    }

    #[test]
    fn test_get_formatted_sql_xml() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT C1 AS ID
            FROM TBL1
            FOR XML RAW('ITEM'), TYPE, ELEMENTS, ROOT('VALUES'), BINARY BASE64
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT C1 AS ID
            FROM TBL1
            FOR XML RAW('ITEM'), TYPE, ELEMENTS, ROOT('VALUES'), BINARY BASE64
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                C1 AS ID
            FROM TBL1
            FOR XML RAW('ITEM'),
                TYPE,
                ELEMENTS,
                ROOT('VALUES'),
                BINARY BASE64"#
        );
    }

    #[test]
    fn test_get_formatted_sql_xml_method() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT T2.Loc.QUERY('.')
            FROM T
            CROSS APPLY Instructions.VALUE('/root/Location') AS T2(Loc)
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT T2.Loc.query('.')
            FROM T
                CROSS APPLY Instructions.value('/root/Location') AS T2(Loc)
"#
        );

        config.newlines = true;

        config.case = ConfigCase::Uppercase;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                T2.Loc.query('.')
            FROM T
                CROSS APPLY Instructions.value('/root/Location') AS T2(Loc)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_keyword_column_name() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT
            T.VALUE AS VALUE, T.[VALUE] AS [VALUE], 'VALUE' AS 'VALUE',
            t.days as days, t.[days] as [days], 'days' as 'days'
            FROM TBL1 AS T
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT
                T.VALUE AS VALUE, T.[VALUE] AS [VALUE], 'VALUE' AS 'VALUE',
                t.days as days, t.[days] as [days], 'days' as 'days'
            FROM TBL1 AS T
"#
        );

        config.newlines = true;
        config.case = ConfigCase::Uppercase;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                T.VALUE AS VALUE,
                T.[VALUE] AS [VALUE],
                'VALUE' AS 'VALUE',
                t.DAYS AS DAYS,
                t.[days] AS [days],
                'days' AS 'days'
            FROM TBL1 AS T"#
        );
    }

    #[test]
    fn test_get_formatted_sql_stuff_comma_list() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            STUFF((SELECT ', ' + C1 FROM TBL1 FOR XML PATH('')), 1, 2, '')
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            STUFF((SELECT ', ' + C1 FROM TBL1 FOR XML PATH('')), 1, 2, '')
"#
        );

        config.newlines = true;
        config.chars = 40;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            STUFF(
                (
                    SELECT
                        ', ' + C1
                    FROM TBL1
                    FOR XML PATH('')
                ),
                1,
                2,
                ''
            )"#
        );
    }

    #[test]
    fn test_get_formatted_sql_table_variable() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            DECLARE TBL1 TABLE (
                C1 UNIQUEIDENTIFIER, C2 UNIQUEIDENTIFIER,
                C3 UNIQUEIDENTIFIER, C4 UNIQUEIDENTIFIER,
                C5 UNIQUEIDENTIFIER, C6 UNIQUEIDENTIFIER
            );
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            DECLARE TBL1 TABLE(
                    C1 UNIQUEIDENTIFIER, C2 UNIQUEIDENTIFIER,
                    C3 UNIQUEIDENTIFIER, C4 UNIQUEIDENTIFIER,
                    C5 UNIQUEIDENTIFIER, C6 UNIQUEIDENTIFIER
                );
"#,
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            DECLARE TBL1 TABLE(
                    C1 UNIQUEIDENTIFIER,
                    C2 UNIQUEIDENTIFIER,
                    C3 UNIQUEIDENTIFIER,
                    C4 UNIQUEIDENTIFIER,
                    C5 UNIQUEIDENTIFIER,
                    C6 UNIQUEIDENTIFIER
                );"#,
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_simple() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"CREATE TABLE TBL1 (C1 INT)"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"CREATE TABLE TBL1(C1 INT)"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"CREATE TABLE TBL1(C1 INT)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_varchar() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(r#"CREATE TABLE TBL1 (C1 VARCHAR(10))"#);

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"CREATE TABLE TBL1(C1 VARCHAR(10))"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"CREATE TABLE TBL1(C1 VARCHAR(10))"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_default() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            CREATE TABLE TBL1 (
                ID UUID NOT NULL DEFAULT UUID()
            )
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            CREATE TABLE TBL1(
                ID UUID NOT NULL DEFAULT UUID()
            )
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            CREATE TABLE TBL1(ID UUID NOT NULL DEFAULT UUID())"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_complex() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            CREATE TABLE IF NOT EXISTS TBL1 (
                ID UUID NOT NULL DEFAULT UUID(),
                C1 VARCHAR(10) NOT NULL,
                D1 DATETIME NULL,
                I1 INT,
                I2 INT, PRIMARY KEY (ID), FOREIGN KEY (I1) REFERENCES TBL2 (ID) ON DELETE CASCADE,
                FOREIGN KEY (I2) REFERENCES TBL3 (ID) ON DELETE SET NULL
            )
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            CREATE TABLE IF NOT EXISTS TBL1(
                ID UUID NOT NULL DEFAULT UUID(),
                C1 VARCHAR(10) NOT NULL,
                D1 DATETIME NULL,
                I1 INT,
                I2 INT, PRIMARY KEY(ID), FOREIGN KEY(I1) REFERENCES TBL2(ID) ON DELETE CASCADE,
                FOREIGN KEY(I2) REFERENCES TBL3(ID) ON DELETE SET NULL
            )
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            CREATE TABLE IF NOT EXISTS TBL1(
                ID UUID NOT NULL DEFAULT UUID(),
                C1 VARCHAR(10) NOT NULL,
                D1 DATETIME NULL,
                I1 INT,
                I2 INT,
                PRIMARY KEY(ID),
                FOREIGN KEY(I1) REFERENCES TBL2(ID) ON DELETE CASCADE,
                FOREIGN KEY(I2) REFERENCES TBL3(ID) ON DELETE SET NULL
            )"#
        );

        config.align_text_groups = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            CREATE TABLE IF NOT EXISTS TBL1(
                ID      UUID        NOT        NULL     DEFAULT UUID(),
                C1      VARCHAR(10) NOT        NULL,
                D1      DATETIME    NULL,
                I1      INT,
                I2      INT,
                PRIMARY KEY(ID),
                FOREIGN KEY(I1)     REFERENCES TBL2(ID) ON      DELETE  CASCADE,
                FOREIGN KEY(I2)     REFERENCES TBL3(ID) ON      DELETE  SET      NULL
            )"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_after_select() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT 1;
            CREATE TABLE TBL1 (
                ID UUID NOT NULL DEFAULT UUID()
            );
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT 1;
            CREATE TABLE TBL1(
                ID UUID NOT NULL DEFAULT UUID()
            );
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                1;

            CREATE TABLE TBL1(ID UUID NOT NULL DEFAULT UUID());"#
        );
    }

    #[test]
    fn test_get_formatted_sql_alter_table() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            ALTER TABLE TBL1 ALTER COLUMN C1 NVARCHAR (20) NOT NULL;
            ALTER TABLE TBL1 DROP COLUMN C1;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            ALTER TABLE TBL1 ALTER COLUMN C1 NVARCHAR(20) NOT NULL;
            ALTER TABLE TBL1 DROP COLUMN C1;
"#,
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            ALTER TABLE TBL1
            ALTER COLUMN C1 NVARCHAR(20) NOT NULL;

            ALTER TABLE TBL1
            DROP COLUMN C1;"#,
        );
    }

    #[test]
    fn test_get_formatted_sql_trigger() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            CREATE TRIGGER IF NOT EXISTS TR1
            AFTER INSERT
            ON TBL1
            FOR EACH ROW
            BEGIN
            CALL SP1(NEW.ID);
            END;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            CREATE TRIGGER IF NOT EXISTS TR1
            AFTER INSERT
                ON TBL1
            FOR EACH ROW
            BEGIN
                CALL SP1(NEW.ID);
            END;
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            CREATE TRIGGER IF NOT EXISTS TR1
            AFTER INSERT ON TBL1
            FOR EACH ROW
            BEGIN
                CALL SP1(NEW.ID);
            END;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_function() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            CREATE FUNCTION FUNC1 ( @P1 NVARCHAR(10) )
            RETURNS NVARCHAR(10)
            WITH EXECUTE AS CALLER AS
            BEGIN RETURN ''; END
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            CREATE FUNCTION FUNC1(@P1 NVARCHAR(10))
            RETURNS NVARCHAR(10)
            WITH EXECUTE AS CALLER AS
            BEGIN RETURN ''; END
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            CREATE FUNCTION FUNC1(@P1 NVARCHAR(10))
            RETURNS NVARCHAR(10) WITH EXECUTE AS CALLER AS
            BEGIN
                RETURN '';
            END"#
        );
    }

    #[test]
    fn test_get_formatted_sql_while_loop() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            DECLARE VAR_COUNT INT;

            SELECT COUNT(ID)
            INTO VAR_COUNT
            FROM TBL1;

            WHILE VAR_COUNT > 0 DO
            DELETE FROM TBL1
            WHERE ID = VAR_COUNT;

            SELECT COUNT(ID)
            INTO VAR_COUNT
            FROM TBL1;
            END WHILE;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            DECLARE VAR_COUNT INT;

            SELECT COUNT(ID)
            INTO VAR_COUNT
            FROM TBL1;

            WHILE VAR_COUNT > 0 DO
                DELETE FROM TBL1
                WHERE ID = VAR_COUNT;

                SELECT COUNT(ID)
                INTO VAR_COUNT
                FROM TBL1;
            END WHILE;
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            DECLARE VAR_COUNT INT;

            SELECT
                COUNT(ID)
            INTO
                VAR_COUNT
            FROM TBL1;

            WHILE VAR_COUNT > 0
            DO
                DELETE
                FROM TBL1
                WHERE ID = VAR_COUNT;

                SELECT
                    COUNT(ID)
                INTO
                    VAR_COUNT
                FROM TBL1;
            END
            WHILE;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_pivot() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            SELECT 'AverageCost' AS CostSortedByProductionDays,
            [0],[1],[2],[3],[4]
            FROM (
            SELECT DaysToManufacture, StandardCost
            FROM Production.Product
            ) AS SourceTable
            PIVOT (
            AVG(StandardCost) FOR DaysToManufacture IN
            ([0],[1],[2],[3],[4])
            ) AS PivotTable;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            SELECT 'AverageCost' AS CostSortedByProductionDays,
                [0], [1], [2], [3], [4]
            FROM (
                    SELECT DaysToManufacture, StandardCost
                    FROM Production.Product
                ) AS SourceTable
            PIVOT (
                    AVG(StandardCost) FOR DaysToManufacture IN
                        ([0], [1], [2], [3], [4])
                ) AS PivotTable;
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                'AverageCost' AS CostSortedByProductionDays,
                [0],
                [1],
                [2],
                [3],
                [4]
            FROM (
                    SELECT
                        DaysToManufacture,
                        StandardCost
                    FROM Production.Product
                ) AS SourceTable
            PIVOT (
                    AVG(StandardCost)
                    FOR DaysToManufacture IN ([0], [1], [2], [3], [4])
                ) AS PivotTable;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_cursor() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            DECLARE @ID INT, @NAME NVARCHAR(50);

            DECLARE SAMPLE_CURSOR CURSOR FOR
            SELECT ID, NAME
            FROM TBL1;

            OPEN SAMPLE_CURSOR

            FETCH NEXT FROM SAMPLE_CURSOR
            INTO @ID, @NAME

            WHILE @@FETCH_STATUS = 0
            BEGIN
            FETCH NEXT FROM SAMPLE_CURSOR
            INTO @VENDOR_ID, @VENDOR_NAME
            END
            CLOSE SAMPLE_CURSOR;
            DEALLOCATE SAMPLE_CURSOR;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            DECLARE @ID INT, @NAME NVARCHAR(50);

            DECLARE SAMPLE_CURSOR CURSOR FOR
            SELECT ID, NAME
            FROM TBL1;

            OPEN SAMPLE_CURSOR

                FETCH NEXT FROM SAMPLE_CURSOR
                INTO @ID, @NAME

                WHILE @@FETCH_STATUS = 0
                BEGIN
                    FETCH NEXT FROM SAMPLE_CURSOR
                    INTO @VENDOR_ID, @VENDOR_NAME
                END
            CLOSE SAMPLE_CURSOR;
            DEALLOCATE SAMPLE_CURSOR;
"#
        );

        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            DECLARE @ID INT,
                @NAME NVARCHAR(50);

            DECLARE SAMPLE_CURSOR CURSOR
            FOR
            SELECT
                ID,
                NAME
            FROM TBL1;

            OPEN SAMPLE_CURSOR
                FETCH NEXT
                FROM SAMPLE_CURSOR
                INTO
                    @ID,
                    @NAME
                WHILE @@FETCH_STATUS = 0
                BEGIN
                    FETCH NEXT
                    FROM SAMPLE_CURSOR
                    INTO
                        @VENDOR_ID,
                        @VENDOR_NAME
                END
            CLOSE SAMPLE_CURSOR;

            DEALLOCATE SAMPLE_CURSOR;"#
        );
    }

    #[test]
    fn test_get_formatted_merge() {
        let mut config: Configuration = Configuration::new();
        let sql: String = String::from(
            r#"
            select 1;
            merge tbl1 as TargetTable using tbl2 as SourceTable on TargetTable.id = @id and TargetTable.c1 = SourceTable.c1
            when not matched by TargetTable then insert (c1) values (SourceTable.c1)
            when not matched by SourceTable and TargetTable.id = @id then delete;
            "#,
        );

        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"
            select 1;
            merge tbl1 as TargetTable using tbl2 as SourceTable on TargetTable.id = @id and TargetTable.c1 = SourceTable.c1
            when not matched by TargetTable then insert (c1) values (SourceTable.c1)
            when not matched by SourceTable and TargetTable.id = @id then delete;
"#
        );

        config.case = ConfigCase::Uppercase;
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, sql.clone()),
            r#"            SELECT
                1;

            MERGE tbl1 AS TargetTable
            USING tbl2 AS SourceTable ON TargetTable.id = @id
                AND TargetTable.c1 = SourceTable.c1
            WHEN NOT MATCHED BY TargetTable THEN
            INSERT (c1)
            VALUES (SourceTable.c1)
            WHEN NOT MATCHED BY SourceTable
                AND TargetTable.id = @id THEN
            DELETE;"#
        );
    }
}
