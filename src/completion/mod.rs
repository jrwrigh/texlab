mod bibtex;
pub mod factory;
mod latex;
mod quality;

use self::bibtex::entry_type::BibtexEntryTypeCompletionProvider;
use self::bibtex::field_name::BibtexFieldNameCompletionProvider;
use self::bibtex::kernel_command::BibtexKernelCommandCompletionProvider;
use self::latex::argument_symbol::LatexArgumentSymbolCompletionProvider;
use self::latex::begin_command::LatexBeginCommandCompletionProvider;
use self::latex::citation::LatexCitationCompletionProvider;
use self::latex::color::LatexColorCompletionProvider;
use self::latex::color_model::LatexColorModelCompletionProvider;
use self::latex::command_symbol::LatexCommandSymbolCompletionProvider;
use self::latex::import::{LatexClassImportProvider, LatexPackageImportProvider};
use self::latex::include::LatexIncludeCompletionProvider;
use self::latex::kernel_command::LatexKernelCommandCompletionProvider;
use self::latex::kernel_environment::LatexKernelEnvironmentCompletionProvider;
use self::latex::label::LatexLabelCompletionProvider;
use self::latex::pgf_library::LatexPgfLibraryCompletionProvider;
use self::latex::tikz_command::LatexTikzCommandCompletionProvider;
use self::latex::tikz_library::LatexTikzLibraryCompletionProvider;
use self::latex::user_command::LatexUserCommandCompletionProvider;
use self::quality::OrderByQualityCompletionProvider;
use crate::feature::{ConcatProvider, FeatureProvider, FeatureRequest};
use futures::prelude::*;
use futures_boxed::boxed;
use itertools::Itertools;
use lsp_types::{CompletionItem, CompletionParams};

pub const COMPLETION_LIMIT: usize = 50;

pub struct CompletionProvider {
    provider: OrderByQualityCompletionProvider<ConcatProvider<CompletionParams, CompletionItem>>,
}

impl CompletionProvider {
    pub fn new() -> Self {
        Self {
            provider: OrderByQualityCompletionProvider::new(ConcatProvider::new(vec![
                Box::new(BibtexEntryTypeCompletionProvider::new()),
                Box::new(BibtexFieldNameCompletionProvider::new()),
                Box::new(BibtexKernelCommandCompletionProvider::new()),
                Box::new(LatexKernelEnvironmentCompletionProvider::new()),
                Box::new(LatexArgumentSymbolCompletionProvider),
                Box::new(LatexPgfLibraryCompletionProvider::new()),
                Box::new(LatexTikzLibraryCompletionProvider::new()),
                Box::new(LatexColorCompletionProvider),
                Box::new(LatexColorModelCompletionProvider::new()),
                Box::new(LatexLabelCompletionProvider),
                Box::new(LatexCitationCompletionProvider),
                Box::new(LatexIncludeCompletionProvider),
                Box::new(LatexClassImportProvider),
                Box::new(LatexPackageImportProvider),
                Box::new(LatexBeginCommandCompletionProvider),
                Box::new(LatexCommandSymbolCompletionProvider),
                Box::new(LatexTikzCommandCompletionProvider::new()),
                Box::new(LatexKernelCommandCompletionProvider::new()),
                Box::new(LatexUserCommandCompletionProvider),
            ])),
        }
    }
}

impl FeatureProvider for CompletionProvider {
    type Params = CompletionParams;
    type Output = Vec<CompletionItem>;

    #[boxed]
    async fn execute<'a>(
        &'a self,
        request: &'a FeatureRequest<CompletionParams>,
    ) -> Vec<CompletionItem> {
        self.provider
            .execute(request)
            .await
            .into_iter()
            .unique_by(|item| item.label.clone())
            .take(COMPLETION_LIMIT)
            .collect()
    }
}
