use texlab_protocol::*;
use texlab_test::formatting::*;

#[tokio::test]
async fn default_settings() {
    let (scenario, edits) = run_bibtex("default/unformatted.bib", None).await;
    assert_eq!(edits.len(), 1);
    assert_eq!(
        edits[0].new_text,
        scenario.read("default/formatted.bib").await
    );
    assert_eq!(edits[0].range, Range::new_simple(0, 0, 0, 52));
}

#[tokio::test]
async fn infinite_line_length() {
    let (scenario, edits) = run_bibtex(
        "infinite_line_length/unformatted.bib",
        Some(BibtexFormattingOptions {
            line_length: Some(0),
        }),
    )
    .await;
    assert_eq!(edits.len(), 1);
    assert_eq!(
        edits[0].new_text,
        scenario.read("infinite_line_length/formatted.bib").await
    );
    assert_eq!(edits[0].range, Range::new_simple(0, 0, 0, 149));
}
