// driver/src/source.rs

use nyanc_core::FileId;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct SourceManager {
    /// 存储所有已加载文件的源代码，使用 Arc 实现高效共享
    files: Vec<Arc<String>>,
    /// 存储从规范化路径到 FileId 的映射，避免重复加载
    paths: HashMap<PathBuf, FileId>,
}

impl SourceManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// 根据路径加载一个源文件。
    /// 如果文件已加载，则直接返回其 FileId；否则，从磁盘读取并分配新 ID。
    pub fn load(&mut self, path: &Path) -> io::Result<FileId> {
        // 将路径规范化，以处理 `.` `..` 等情况
        let canonical_path = path.canonicalize()?;
        
        if let Some(file_id) = self.paths.get(&canonical_path) {
            return Ok(*file_id);
        }

        let source_text = fs::read_to_string(&canonical_path)?;
        let file_id: FileId = self.files.len();

        self.files.push(Arc::new(source_text));
        self.paths.insert(canonical_path, file_id);
        
        Ok(file_id)
    }

    /// 根据 FileId 获取文件的源代码
    pub fn source_text(&self, file_id: FileId) -> Arc<String> {
        // Arc::clone 只会增加引用计数，开销极小
        self.files[file_id].clone()
    }

    /// 根据 FileId 获取文件的路径
    pub fn path(&self, file_id: FileId) -> Option<&PathBuf> {
        self.paths
            .iter()
            // 闭包的参数 `item` 的类型是 &(&PathBuf, &FileId)
            // item.1 的类型是 &FileId
            // *item.1 解引用后，得到 FileId，可以与传入的 file_id 比较
            .find(|item| *item.1 == file_id)
            // find 成功后，返回的 `item` 类型依然是 &(&PathBuf, &FileId)
            // item.0 是 &PathBuf
            .map(|item| item.0)
    }
}