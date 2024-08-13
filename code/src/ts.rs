use crate::ast::{FunctionDef, IfStatement, Statement, WhileStatement};

impl<'a> FunctionDef<'a> {
    pub fn to_typescript(&self) -> String {
        let mut ts_code = format!("function {}({}) {{\n", self.name, self.params);

        for (i, stmt) in self.body.iter().enumerate() {
            match stmt {
                Statement::Assignment { var, expr } => {
                    ts_code.push_str(&format!("    let {} = {};\n", var, expr));
                }
                Statement::IfStatement(if_stmt) => {
                    ts_code.push_str(&if_stmt.to_typescript(1));
                }
                Statement::WhileStatement(while_stmt) => {
                    ts_code.push_str(&while_stmt.to_typescript(1));
                }
                Statement::Expression(expr) => {
                    if i == self.body.len() - 1 {
                        ts_code.push_str(&format!("    return {};\n", expr));
                    } else {
                        ts_code.push_str(&format!("    {};\n", expr));
                    }
                }
            }
        }

        ts_code.push_str("}\n");
        ts_code
    }
}

impl<'a> IfStatement<'a> {
    fn to_typescript(&self, indent: usize) -> String {
        let indent_str = "    ".repeat(indent);
        let mut ts_code = format!("{}if ({}) {{\n", indent_str, self.condition);

        for stmt in &self.then_branch {
            ts_code.push_str(&stmt.to_typescript(indent + 1));
        }

        ts_code.push_str(&format!("{}}}\n", indent_str));

        if let Some(else_branch) = &self.else_branch {
            ts_code.push_str(&format!("{}else {{\n", indent_str));
            for stmt in else_branch {
                ts_code.push_str(&stmt.to_typescript(indent + 1));
            }
            ts_code.push_str(&format!("{}}}\n", indent_str));
        }

        ts_code
    }
}

impl<'a> WhileStatement<'a> {
    fn to_typescript(&self, indent: usize) -> String {
        let indent_str = "    ".repeat(indent);
        let mut ts_code = format!("{}while ({}) {{\n", indent_str, self.condition);

        for stmt in &self.body {
            ts_code.push_str(&stmt.to_typescript(indent + 1));
        }

        ts_code.push_str(&format!("{}}}\n", indent_str));
        ts_code
    }
}

impl<'a> Statement<'a> {
    fn to_typescript(&self, indent: usize) -> String {
        let indent_str = "    ".repeat(indent);
        match self {
            Statement::Assignment { var, expr } => {
                format!("{}let {} = {};\n", indent_str, var, expr)
            }
            Statement::IfStatement(if_stmt) => if_stmt.to_typescript(indent),
            Statement::WhileStatement(while_stmt) => while_stmt.to_typescript(indent),
            Statement::Expression(expr) => format!("{}{};\n", indent_str, expr),
        }
    }
}
