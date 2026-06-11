pub fn small_signal_param_name(
    param: &str,
    instance: &str,
    branch: &str,
) -> String {
    format!(
        "{}__{}__{}",
        sanitize_name(param),
        sanitize_name(instance),
        sanitize_name(branch)
    )
}

pub fn small_signal_element_name(
    spice_prefix: &str,
    param: &str,
    instance: &str,
    branch: &str,
) -> String {
    format!(
        "{}_{}",
        sanitize_name(spice_prefix),
        small_signal_param_name(param, instance, branch)
    )
}

fn sanitize_name(value: &str) -> String {
    value.trim().replace('-', "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_small_signal_param_name() {
        assert_eq!(small_signal_param_name("gm", "xdp", "m1"), "gm__xdp__m1");
        assert_eq!(small_signal_param_name("ro", "xcm", "m2"), "ro__xcm__m2");
    }

    #[test]
    fn builds_small_signal_element_name() {
        assert_eq!(
            small_signal_element_name("G", "gm", "xdp", "m1"),
            "G_gm__xdp__m1"
        );
        assert_eq!(
            small_signal_element_name("R", "ro", "xdp", "m1"),
            "R_ro__xdp__m1"
        );
    }

    #[test]
    fn sanitizes_basic_separators() {
        assert_eq!(
            small_signal_param_name("g-m", "x-dp", "m-1"),
            "g_m__x_dp__m_1"
        );
    }
}
