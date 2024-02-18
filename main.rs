use std::io;

#[derive(Debug, Clone, PartialEq)]
struct ProofStep {
    deduction: LogicalExpr,
    based_on: Vec<LogicalExpr>,
    rule_applied: Rule,
}

#[derive(Debug, Clone, PartialEq)]
enum Rule {
    LawOfSyllogism,
    ModusPonens,
    ModusTollens,
}

#[derive(Debug, Clone, PartialEq)]
enum LogicalExpr {
    Implication(String, String),
    Negation(String),
    Conjunction(String, String),
    Disjunction(String, String),
    Conclusion(String),
}

impl LogicalExpr {
    fn get_subject(&self) -> Option<&String> {
        match self {
            LogicalExpr::Implication(p, _) => Some(p),
            LogicalExpr::Negation(p) => Some(p),
            LogicalExpr::Conjunction(p, _) => Some(p),
            LogicalExpr::Disjunction(p, _) => Some(p),
            LogicalExpr::Conclusion(_) => None,
        }
    }
}

fn parse_expression(input: &str) -> Option<LogicalExpr> {
    let input = input.trim();
    if let Some(index) = input.find('>') {
        let parts = input.split_at(index);
        Some(LogicalExpr::Implication(parts.0.trim().to_string(), parts.1[1..].trim().to_string()))
    } else if input.starts_with("~") {
        Some(LogicalExpr::Negation(input[1..].trim().to_string()))
    } else if let Some(index) = input.find('*') {
        let parts = input.split_at(index);
        Some(LogicalExpr::Conjunction(parts.0.trim().to_string(), parts.1[1..].trim().to_string()))
    } else if let Some(index) = input.find('+') {
        let parts = input.split_at(index);
        Some(LogicalExpr::Disjunction(parts.0.trim().to_string(), parts.1[1..].trim().to_string()))
    } else if input.starts_with("R ") || input.starts_with("R") {
        let conclusion = input.trim_start_matches('R').trim().to_string();
        Some(LogicalExpr::Conclusion(conclusion))
    } else if !input.is_empty() {
        Some(LogicalExpr::Conclusion(input.to_string()))
    } else {
        None
    }
}

fn apply_deduction_rules_and_validate(expressions: &[LogicalExpr], conclusion: &LogicalExpr) -> (bool, Vec<ProofStep>) {
    let mut deductions: Vec<LogicalExpr> = Vec::new();
    let mut proof_steps: Vec<ProofStep> = Vec::new();

    // Iterate through expressions to apply deduction rules
    for expr in expressions {
        match expr {
            LogicalExpr::Implication(p, q) => {
                // Check for Modus Ponens
                if expressions.contains(&LogicalExpr::Conclusion(p.clone())) || deductions.contains(&LogicalExpr::Conclusion(p.clone())) {
                    let deduction = LogicalExpr::Conclusion(q.clone());
                    if !deductions.contains(&deduction) {
                        deductions.push(deduction.clone());
                        proof_steps.push(ProofStep {
                            deduction: deduction.clone(),
                            based_on: vec![expr.clone()],
                            rule_applied: Rule::ModusPonens,
                        });
                    }
                }

                // Check for Modus Tollens
                let negation_q = LogicalExpr::Negation(q.clone());
                if expressions.contains(&negation_q) || deductions.contains(&negation_q) {
                    let deduction = LogicalExpr::Negation(p.clone());
                    if !deductions.contains(&deduction) {
                        deductions.push(deduction.clone());
                        proof_steps.push(ProofStep {
                            deduction: deduction.clone(),
                            based_on: vec![expr.clone(), negation_q],
                            rule_applied: Rule::ModusTollens,
                        });
                    }
                }

                // Apply the Law of Syllogism
                for other_expr in expressions {
                    if let LogicalExpr::Implication(q2, r) = other_expr {
                        if q == q2 && !deductions.contains(&LogicalExpr::Implication(p.clone(), r.clone())) {
                            let new_deduction = LogicalExpr::Implication(p.clone(), r.clone());
                            deductions.push(new_deduction.clone());
                            proof_steps.push(ProofStep {
                                deduction: new_deduction,
                                based_on: vec![expr.clone(), other_expr.clone()],
                                rule_applied: Rule::LawOfSyllogism,
                            });
                        }
                    }
                }
            },
            _ => {}
        }    
    }

    let direct_conclusion_valid = deductions.iter().any(|d| d == conclusion);
    let conclusion_supported_by_backward_chaining = backward_chain(expressions, conclusion);

    let conclusion_valid = direct_conclusion_valid || conclusion_supported_by_backward_chaining;

    (conclusion_valid, proof_steps)
}

fn backward_chain(expressions: &[LogicalExpr], target: &LogicalExpr) -> bool {
    if expressions.contains(target) {
        return true;
    }

    expressions.iter().any(|expr| {
        if let LogicalExpr::Implication(p, q) = expr {
            if let LogicalExpr::Conclusion(target_str) = target {
                if q == target_str {
                    return backward_chain(expressions, &LogicalExpr::Conclusion(p.clone()));
                }
            }
        }
        false
    })
}

fn main() {
    let mut expressions = Vec::new();
    let mut input = String::new();

    println!("Enter logical statements, then type 'DONE' when finished:");
    while input.trim().to_lowercase() != "done" {
        println!("Please enter a logical expression or 'DONE' to finish:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Some(expr) = parse_expression(&input) {
            expressions.push(expr);
            println!("Expression added.");
        } else if input.trim().to_lowercase() != "done" {
            println!("Invalid statement or format. Please re-enter.");
        }
    }

    if let Some(conclusion) = expressions.last() {
        let (is_valid, proof_steps) = apply_deduction_rules_and_validate(&expressions, conclusion);
        if is_valid {
            println!("The conclusion is valid. Here is the proof:");
            for step in proof_steps {
                println!("Deduction: {:?}", step.deduction);
                println!("Based on: {:?}", step.based_on.iter().map(|expr| format!("{:?}", expr)).collect::<Vec<String>>().join(", "));
                println!("Rule Applied: {:?}", step.rule_applied);
                println!("--------------------------------");
            }
        } else {
            println!("The conclusion could not be validated based on the provided premises.");
        }
    } else {
        println!("No conclusion provided.");
    }
}