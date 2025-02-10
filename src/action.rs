use crate::context::Context;

pub type Action = fn(Context) -> Result<(), Box<dyn std::error::Error>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flag::{Flag, FlagKind};

    #[test]
    fn test_action() {
        let action: Action = |ctx: Context| {
            let flag = ctx.value_flag("flag").unwrap();
            assert_eq!(flag, "value");
            Ok(())
        };

        let ctx = Context::new(
            vec![
                "test".to_string(),
                "--flag".to_string(),
                "value".to_string(),
            ],
            vec![Flag::new("flag", FlagKind::Value)],
        );
        action(ctx).unwrap();
    }
}
