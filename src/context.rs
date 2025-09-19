// 从兄弟 Crate 中导入我们需要的“工具”和“数据”
use crate::interner::Interner;
use crate::source::SourceManager;
use reporter::DiagnosticsEngine; 
use analyzer::ty::TypeMap; 
use nyanc_core::FileId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

// Context 在最高层被定义，因为它需要了解所有子系统
pub struct CompilationContext {
    pub diagnostics: DiagnosticsEngine,
    pub interner: RefCell<Interner>,
    pub type_map: TypeMap, 
    pub source_manager: RefCell<SourceManager>,

    // AST 缓存：按需解析，并将结果缓存起来
    // 缓存现在存储解析一个文件的完整结果：(AST仓库, 模块根节点ID)
    pub ast_cache: RefCell<HashMap<FileId, (Arc<ast::Ast>, ast::AstId<ast::Module>)>>,
}

impl CompilationContext {
    pub fn new() -> Self {
        Self {
            diagnostics: DiagnosticsEngine::new(),
            interner: RefCell::new(Interner::new()),
            type_map: TypeMap::new(), 
            source_manager: RefCell::new(SourceManager::new()),
            ast_cache: RefCell::new(HashMap::new()),
        }
    }
}
