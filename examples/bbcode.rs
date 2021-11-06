use desumark::*;
use std::error::Error as StdError;

fn main() -> Result<(), Box<dyn StdError>> {
    let layout = Layout::<u32>::new()
        .pattern(
            "b",
            Pattern::new()
                .segment(Segment::Static("<b>"))
                .segment(Segment::Inner)
                .segment(Segment::Static("</b>")),
        )?
        .pattern(
            "i",
            Pattern::new()
                .segment(Segment::Static("<i>"))
                .segment(Segment::Inner)
                .segment(Segment::Static("</i>")),
        )?
        .pattern(
            "textwall",
            Pattern::new()
                .segment(Segment::Static("<div class=\"textwall\"><b>"))
                .segment(Segment::Linked(String::from("title")))
                .segment(Segment::Static("</b>"))
                .segment(Segment::Inner)
                .segment(Segment::Static("</div>")),
        )?;

    let source = "[textwall title=\"Title asd\"][/textwall]";

    let processed = layout.proccess(source).unwrap();
    println!("{:?}", processed);

    Ok(())
}
