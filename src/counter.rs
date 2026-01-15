use typst::introspection::Introspector;
use typst::syntax::FileId;

pub struct Count {
    pub words: usize,
    pub characters: usize,
}

pub fn count_document(
    introspector: &Introspector,
    exclude_imports: bool,
    main_file_id: FileId,
) -> Count {
    let mut words = 0;
    let mut characters = 0;

    for element in introspector.all() {
        if exclude_imports
            && let Some(file_id) = element.span().id()
            && file_id != main_file_id
        {
            continue;
        }

        let text = element.plain_text();
        if !text.is_empty() {
            characters += text.chars().count();
            words += text.split_whitespace().count();
        }
    }

    Count { words, characters }
}
