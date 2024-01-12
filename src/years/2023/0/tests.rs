use lazy_static::lazy_static;

use crate::main_shim;
use crate::tests::ARGS_LOCK_MUTEX;
use crate::{
    clap::{override_args, Args},
    types::PartEnum,
};

lazy_static! {
    static ref DEFAULT_ARGS: Args = {
        let mut args = Args::new();
        args.year = super::super::YEAR.year;
        args.day = super::DAY.day;
        args.example = true;
        return args;
    };
}

fn get_args(part: PartEnum) -> Args {
    let mut args = DEFAULT_ARGS.clone();
    args.part = part;
    return args;
}

#[test]
fn test_day0p1() -> Result<(), String> {
    return ARGS_LOCK_MUTEX.lock().unwrap().run(|| {
        let args = get_args(PartEnum::PartOne);
        override_args(args);

        let output = main_shim();
        assert_eq!(output, "0");
        return Ok(());
    });
}
