pub mod context;
pub mod interner;
pub mod source;

use crate::context::CompilationContext;
use analyzer::db::AnalyzerDb;
use ast::Path as AstPath; // 使用 `as` 避免与 std::path::Path 混淆
use lexer::Lexer;
use nyanc_core::{FileId, Symbol};
use parser::Parser;
use std::sync::Arc;

/// 为我们的“具体数据库”实现 analyzer 的“抽象契约”
impl AnalyzerDb for CompilationContext {
    /// 按需获取 AST
    fn ast(&self, file_id: FileId) -> Arc<ast::Module> {
        // 1. 先检查缓存中是否已有
        if let Some(ast) = self.ast_cache.borrow().get(&file_id) {
            return ast.clone();
        }

        // 2. 如果没有，就从 SourceManager 获取源码
        let source_text = self.source_manager.borrow().source_text(file_id);
        
        // 3. 运行 Lexer 和 Parser
        let lexer = Lexer::new(&source_text, file_id, &self.diagnostics);
        let mut parser = Parser::new(lexer, &self.diagnostics);
        let ast = parser.parse();
        
        // 4. 将新生成的 AST 存入缓存并返回
        let arc_ast = Arc::new(ast);
        self.ast_cache.borrow_mut().insert(file_id, arc_ast.clone());
        
        arc_ast
    }

    /// 解析模块路径
    fn resolve_module(&self, anchor_file: FileId, path: &AstPath) -> Option<FileId> {
        // 获取当前文件的目录路径
        let binding = self.source_manager.borrow();
        let anchor_path = binding.path(anchor_file)?.parent()?;
        
        // 拼接出要查找的模块的完整路径
        // TODO: 这是一个非常简化的实现，未来需要支持更复杂的模块查找规则
        let mut module_path = anchor_path.to_path_buf();
        // 简单地将路径段拼接起来
        for segment in &path.segments {
            module_path.push(&segment.lexeme);
        }
        module_path.set_extension("ny");

        // 尝试加载这个文件
        match self.source_manager.borrow_mut().load(&module_path) {
            Ok(file_id) => Some(file_id),
            Err(_) => {
                // TODO: 报告一个“模块未找到”的错误
                None
            }
        }
    }

    fn intern_string(&self, s: &str) -> Symbol {
        // 通过 RefCell 的 borrow_mut() 获取可变借用，并调用 interner 的方法
        self.interner.borrow_mut().intern(s)
    }
}