// nyanc/src/interner.rs

use nyanc_core::Symbol;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Interner {
    /// 用于从 String 快速查到 Symbol
    map: HashMap<String, Symbol>,
    /// 用于从 Symbol 快速查回到 &str
    vec: Vec<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self::default()
    }

    /// 将一个字符串驻留，返回其唯一的 Symbol。
    pub fn intern(&mut self, s: &str) -> Symbol {
        if let Some(symbol) = self.map.get(s) {
            return *symbol;
        }

        // 如果字符串不存在，创建一个新的 Symbol
        let symbol = Symbol(self.vec.len() as u32);
        let s = s.to_string(); // 拥有字符串的所有权
        self.vec.push(s.clone());
        self.map.insert(s, symbol);
        symbol
    }

    /// 根据 Symbol 查找回原始的字符串切片。
    pub fn lookup(&self, symbol: Symbol) -> &str {
        &self.vec[symbol.0 as usize]
    }
}