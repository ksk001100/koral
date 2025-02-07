use crate::context::Context;

pub type Action = fn(Context) -> Result<(), Box<dyn std::error::Error>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use crate::flag::{Flag, FlagKind};

    #[test]
    fn test_action() {
        let action = |ctx: Context| {
            let flag = ctx.value_flag("flag").unwrap();
            assert_eq!(flag, "value");
            Ok(())
        };

        let app = App::new("test")
            .flag(Flag::new("flag", FlagKind::Value))
            .action(action);
        let ctx = Context::new(
            &app,
            vec![
                "test".to_string(),
                "--flag".to_string(),
                "value".to_string(),
            ],
        );
        action(ctx).unwrap();
    }
}
