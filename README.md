

## Note: this repo has a very specific purpose

This repo contains a modified version of https://github.com/bodoni/svg that removes any floats and the parser. Its intended use is for building SVGs in a Radix (radixdlt.com) blueprint where floats are not allowed and the parser is not required.

---

The package provides an SVG composer and parser.

## Example: Composing

```rust
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

let data = Data::new()
    .move_to((10, 10))
    .line_by((0, 50))
    .line_by((50, 0))
    .line_by((0, -50))
    .close();

let path = Path::new()
    .set("fill", "none")
    .set("stroke", "black")
    .set("stroke-width", 3)
    .set("d", data);

let document = Document::new()
    .set("viewBox", (0, 0, 70, 70))
    .add(path);

svg::save("image.svg", &document).unwrap();
```
