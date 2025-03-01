use crate::utils::node_reader::read_file;
use compact_str::CompactString;
use serde::Deserialize;
extern crate alloc;
use alloc::{sync::Arc, vec::Vec};
use anyhow::Result;
use once_cell::sync::{Lazy, OnceCell};

pub static PROFILE: Lazy<Config> = Lazy::new(|| {
    let profile = read_file::<65536>(b"./thread_opt.toml\0").unwrap();
    #[cfg(debug_assertions)]
    log::debug!("{profile:?}");

    let profile: Config = toml::from_str(&profile).unwrap();
    #[cfg(debug_assertions)]
    for i in &profile.comm_match {
        for j in &i.packages {
            log::info!("{j}");
        }
    }
    profile
});

#[derive(Deserialize)]
pub struct Config {
    pub comm_match: Vec<NameMatch>,
}

#[derive(Deserialize)]
pub struct NameMatch {
    pub packages: Vec<CompactString>,
    pub policy: Policy,
}

type ByteArray = heapless::Vec<u8, 16>;
#[derive(Deserialize)]
pub struct Policy {
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub top: Vec<ByteArray>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub only6: Vec<ByteArray>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub only7: Vec<ByteArray>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub middle: Vec<ByteArray>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub background: Vec<ByteArray>,
}

pub fn init_packages(vec: &[CompactString]) -> &'static [&'static str] {
    static CACHE: OnceCell<Arc<[&'static str]>> = OnceCell::new();

    // 获取或初始化缓存
    let cached = CACHE.get_or_init(|| {
        // 将 CompactString 转换为 &'static str
        // 安全条件：确保 CompactString 生命周期足够长
        let static_slices: Vec<&'static str> = vec
            .iter()
            .map(|cs| unsafe { core::mem::transmute::<&str, &'static str>(cs.as_str()) })
            .collect();
        // 通过 Arc 共享内存
        Arc::from(static_slices.into_boxed_slice())
    });
    &cached[..]
}

fn deserialize_byte_array<'de, D>(deserializer: D) -> Result<Vec<heapless::Vec<u8, 16>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Vec<CompactString> = Vec::deserialize(deserializer)?;
    let mut result = Vec::new();
    for s in strings {
        let bytes = s.as_bytes();
        let vec = heapless::Vec::from_slice(bytes)
            .map_err(|()| serde::de::Error::custom("String exceeds capacity"))?;
        result.push(vec);
    }
    Ok(result)
}
