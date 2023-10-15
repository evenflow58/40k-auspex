pub fn as_vec<T>(val: Option<&AttributeValue>) -> Vec<T> {
    if let Some(val) = val {
        if let Ok(val) = val.as_l() {
            return val
                .iter()
                .map(|v| v.into())
                .collect();
        }
    }
    vec![]
}