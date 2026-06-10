use std::fmt::Display;
use std::collections::HashMap;

pub fn pretty_system<T>(
    a: &[Vec<T>],
    x: &[T],
    z: &[T],
) -> String
where
    T: Display,
{
    let mut out = String::new();

    out.push_str("MNA system\n");
    out.push_str("==========\n\n");

    if a.is_empty() {
        out.push_str("<empty system>\n");
        return out;
    }

    for (row_idx, row) in a.iter().enumerate() {
        let mut terms: Vec<String> = Vec::new();

        for (col_idx, coeff) in row.iter().enumerate() {
            let Some(var) = x.get(col_idx) else {
                continue;
            };

            let coeff_str = coeff.to_string();

            // Opcional: no mostrar términos con coeficiente 0
            if coeff_str == "0" || coeff_str == "0.0" {
                continue;
            }

            let term = if coeff_str == "1" || coeff_str == "1.0" {
                format!("{var}")
            } else if coeff_str == "-1" || coeff_str == "-1.0" {
                format!("-{var}")
            } else {
                format!("({coeff})·{var}")
            };

            terms.push(term);
        }

        let lhs = if terms.is_empty() {
            "0".to_string()
        } else {
            terms.join(" + ")
        };

        let rhs = z
            .get(row_idx)
            .map(|expr| expr.to_string())
            .unwrap_or_else(|| "<missing rhs>".to_string());

        out.push_str(&format!("eq_{}: {} = {}\n", row_idx + 1, lhs, rhs));
    }

    out
}

pub fn pretty_solutions<T>(solutions: &HashMap<String, T>) -> String
where
    T: Display,
{
    let mut out = String::new();

    out.push_str("MNA solutions\n");
    out.push_str("=============\n\n");

    if solutions.is_empty() {
        out.push_str("<empty solutions>\n");
        return out;
    }

    let mut entries: Vec<(&String, &T)> = solutions.iter().collect();

    entries.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

    for (name, expr) in entries {
        out.push_str(&format!("{name} = {expr}\n"));
    }

    out
}
