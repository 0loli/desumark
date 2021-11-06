## Description
A little library for parsing bbcode tags.
This library supports options(with custom handlers too), short closing.

## Install
Add line below to dependencies in your project's Cargo.toml file
```toml
desumark = "0.1"
```

## Example
```rust
use desumark::{Error as MarkError, Layout, Pattern, Segment};
struct OptionIsNotNumber;

let layout = Layout::<u32>::new()
    .pattern(
        "b",
        Pattern::new()
            .segment(Segment::Static("<b>"))
            .segment(Segment::Inner)
            .segment(Segment::Static("</b>")),
    ).unwrap() // unwrap is needed because .pattern throws error if the pattern is already present
    .pattern(
        "i",
        Pattern::new()
            .segment(Segment::Static("<i>"))
            .segment(Segment::Inner)
            .segment(Segment::Static("</i>")),
    ).unwrap()
    .pattern(
        "textwall",
        Pattern::new()
            .segment(Segment::Static("<div class=\"textwall\"><b>"))
            .segment(Segment::Linked(String::from("title")))
            .segment(Segment::Static("</b>"))
            .segment(Segment::Inner)
            .segment(Segment::Static("</div>")),
    ).unwrap()
    .pattern(
    	"double-num",
    	Pattern::new()
    		.segment(Segment::Computed(|ctx| => {
    			let number: u32 = match ctx.args.get("number") {
    				Some(v) => v.parse().map_err(MarkError::Custom(OptionIsNotNumber))?,
    				None => 0
    			};

    			return Ok((number * 2).to_string())
    		}))
    ).unwrap();
```