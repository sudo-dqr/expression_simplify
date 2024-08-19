pub mod simplifier_module {

    pub fn presimplify(expression: &mut String) {
        delete_space(expression);
        delete_continious_add_sub(expression);
        delete_pre_add(expression);
        delete_pre_zero(expression);
        sort_minus(expression);
    }

    pub fn postsimplify(expression: &mut String) {
        delete_continious_add_sub(expression);
        sort_minus(expression);
        delete_pre_add(expression);
    }

    fn delete_space(expression: &mut String) {
        expression.retain(|c| !c.is_whitespace());
    }

    fn delete_continious_add_sub(expression: &mut String) {
        let mut i = 0;
        while i < expression.len() {
            if expression.chars().nth(i).unwrap() == '+' 
            || expression.chars().nth(i).unwrap() == '-' {
                let mut tail = i;
                let mut minus_cnt = 0;
                while tail < expression.len() && 
                    (expression.chars().nth(tail).unwrap() == '+' || expression.chars().nth(tail).unwrap() == '-') {
                    if expression.chars().nth(tail).unwrap() == '-' {
                        minus_cnt += 1;
                    }
                    tail += 1;
                }
                if minus_cnt % 2 == 1 {
                    expression.replace_range(i..tail, "-");
                } else {
                    expression.replace_range(i..tail, "+");
                }
            }
            i += 1;
        }
    }

    fn delete_pre_add(expression: &mut String) {
        // 1. 首位是加号
        if expression.starts_with('+') {
            expression.remove(0);
        }

        // 2. 乘法后的加号
        for i in 0..expression.len() - 1 {
            if expression.chars().nth(i).unwrap() == '*' &&
                expression.chars().nth(i + 1).unwrap() == '+' {
                expression.remove(i + 1);
            }
        }

        // 3. 左括号后的加号
        for i in 0..expression.len() - 1 {
            if expression.chars().nth(i).unwrap() == '(' &&
                expression.chars().nth(i + 1).unwrap() == '+' {
                expression.remove(i + 1);
            }
        }

        // 4. 指数中的正号
        for i in 0..expression.len() - 1 {
            if expression.chars().nth(i).unwrap() == '^' &&
                expression.chars().nth(i + 1).unwrap() == '+' {
                expression.remove(i + 1);
            }
        }
    }

    fn delete_pre_zero(expression: &mut String) {
        let mut i = 0;
        while i < expression.len() {
            if expression.chars().nth(i).unwrap() == '0' && 
            (i == 0 || !expression.chars().nth(i - 1).unwrap().is_ascii_digit()) {
                let mut tail1 = i;
                while tail1 < expression.len() && expression.chars().nth(tail1).unwrap() == '0' {
                    tail1 += 1;
                }
                let zero_length = tail1 - i;
                let mut tail2 = i;
                while tail2 < expression.len() && expression.chars().nth(tail2).unwrap().is_ascii_digit() {
                    tail2 += 1;
                }
                let number_length = tail2 - i;
                match zero_length.cmp(&number_length) {
                    std::cmp::Ordering::Less => {
                        expression.replace_range(i..tail1, "");
                    },
                    std::cmp::Ordering::Equal => {
                        expression.replace_range(i..tail1, "0");
                    },
                    std::cmp::Ordering::Greater => continue,
                }
            }
            i += 1;
        }
    }

    fn sort_minus(expression: &mut String) {
        if expression.starts_with('-') {
            let mut add_pos = 0;
            for i in 1..expression.len() {
                if expression.chars().nth(i).unwrap() == '+' {
                    add_pos = i;
                    break;
                }
            }
            if add_pos != 0 {
                for i in add_pos+1..expression.len() {
                    if expression.chars().nth(i).unwrap() == '-' ||
                        expression.chars().nth(i).unwrap() == '+' {
                            break;
                    }
                }
                let mut new_expression = String::new();
                new_expression.push_str(&expression[add_pos..]);
                new_expression.push_str(&expression[0..add_pos]);
                *expression = new_expression;
            }
        }
    }


}