use super::{policy_mingchao, policy_uname, policy_unity};
use libc::pid_t;
use std::sync::LazyLock;
const UNITY: [&str; 7] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
    "com.papegames.infinitynikki",
];

const UNAME: [&str; 2] = ["com.tencent.tmgp.pubgmhd", "com.netease.yyslscn"];

const MINGCHAO: [&str; 1] = ["com.kurogame.mingchao"];

type ConfigTuple<'a> = (&'a [&'a str], fn(pid_t, &str));
pub static PACKAGE_CONFIGS: LazyLock<[ConfigTuple; 3]> = LazyLock::new(|| {
    [
        (&UNITY[..], policy_unity::start_task as fn(pid_t, &str)),
        (&UNAME[..], policy_uname::start_task as fn(pid_t, &str)),
        (
            &MINGCHAO[..],
            policy_mingchao::start_task as fn(pid_t, &str),
        ),
    ]
});
