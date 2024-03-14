use crate::prelude::*;

pub(crate) fn page_index(contexts: &Contexts) -> Html {
    push_state("/software-engineer-standards-and-practices");
    page_home(contexts)
}

/// App home page
pub(crate) fn page_home(_contexts: &Contexts) -> Html {
    set_title(
        "Software Engineer Standards & Practices for Agile Development and Continuous Delivery",
    );
    html! {
        <>
            <MarkdownContent href="/d/en-US/home.md" />
            <NextPageButton url="/about" />
        </>
    }
}
