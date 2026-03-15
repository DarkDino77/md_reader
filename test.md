# The Complete Markdown Test Document

A long document for testing scrolling, rendering, font sizes, and all markdown features.

---

## Table of Contents

1. [Headings](#headings)
2. [Text Formatting](#text-formatting)
3. [Lists](#lists)
4. [Code](#code)
5. [Tables](#tables)
6. [Blockquotes](#blockquotes)
7. [Links](#links)
8. [Long Prose](#long-prose)

---

## Headings

# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

---

## Text Formatting

This is **bold text** and this is *italic text* and this is ***bold and italic***.

This is `inline code` inside a sentence.

This is ~~strikethrough~~ text.

This is normal text followed by a line break.
This is the next line.

This is a new paragraph entirely separated by a blank line.

Here is some text with a lot of words to test line wrapping behavior. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

---

## Lists

### Unordered List

- Item one
- Item two
- Item three
  - Nested item A
  - Nested item B
    - Deeply nested item
    - Another deeply nested item
  - Nested item C
- Item four
- Item five

### Ordered List

1. First item
2. Second item
3. Third item
   1. Sub item one
   2. Sub item two
4. Fourth item
5. Fifth item

### Mixed List

- Unordered item
  1. Ordered sub item
  2. Another ordered sub item
- Another unordered item
  - Nested unordered
    1. Deep ordered

### Task List

- [x] Completed task
- [x] Another completed task
- [ ] Incomplete task
- [ ] Another incomplete task
- [x] Done
- [ ] Not done

---

## Code

### Inline Code

Use `cargo run` to run your project. The `fn main()` function is the entry point.

### Rust Code Block

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Document {
    title: String,
    content: String,
    path: Option<std::path::PathBuf>,
    font_size: f32,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            content: String::new(),
            path: None,
            font_size: 16.0,
        }
    }
}

impl Document {
    fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            ..Default::default()
        }
    }

    fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
}

fn main() {
    let mut docs: HashMap<u32, Document> = HashMap::new();
    docs.insert(1, Document::new("Hello World"));
    docs.insert(2, Document::new("My Notes"));

    for (id, doc) in &docs {
        println!("{}: {} ({} words)", id, doc.title, doc.word_count());
    }
}
```

### Python Code Block

```python
def fibonacci(n: int) -> list[int]:
    sequence = [0, 1]
    for i in range(2, n):
        sequence.append(sequence[i-1] + sequence[i-2])
    return sequence[:n]

result = fibonacci(10)
print(result)  # [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

### Bash Code Block

```bash
# Build and run a Rust project
cargo new my_project
cd my_project
cargo add serde --features derive
cargo add tokio --features full
cargo build --release
./target/release/my_project
```

---

## Tables

### Simple Table

| Name       | Age | City        |
|------------|-----|-------------|
| Alice      | 30  | New York    |
| Bob        | 25  | London      |
| Charlie    | 35  | Tokyo       |
| Diana      | 28  | Sydney      |
| Eve        | 32  | Berlin      |

### Alignment Table

| Left Aligned | Center Aligned | Right Aligned |
|:-------------|:--------------:|--------------:|
| Left         | Center         | Right         |
| Text         | Text           | Text          |
| More text    | More text      | More text     |
| 100          | 200            | 300           |

### Rust Types Table

| Type    | Size    | Min Value            | Max Value            |
|---------|---------|----------------------|----------------------|
| `i8`    | 1 byte  | -128                 | 127                  |
| `i16`   | 2 bytes | -32,768              | 32,767               |
| `i32`   | 4 bytes | -2,147,483,648       | 2,147,483,647        |
| `i64`   | 8 bytes | -9.2 × 10^18         | 9.2 × 10^18          |
| `u8`    | 1 byte  | 0                    | 255                  |
| `u32`   | 4 bytes | 0                    | 4,294,967,295        |
| `f32`   | 4 bytes | -3.4 × 10^38         | 3.4 × 10^38          |
| `f64`   | 8 bytes | -1.8 × 10^308        | 1.8 × 10^308         |

---

## Blockquotes

> This is a simple blockquote. It can span multiple lines and will be rendered with a special style to distinguish it from regular text.

> **Note:** This is an important blockquote with bold text inside it.
> It continues on the next line.
> And the line after that.

> Nested blockquotes:
>
> > This is a nested blockquote inside another blockquote.
> >
> > > And this is a third level of nesting.

---

## Links

[Rust Official Website](https://www.rust-lang.org)

[egui GitHub](https://github.com/emilk/egui)

[The Rust Book](https://doc.rust-lang.org/book/)

[Cargo Documentation](https://doc.rust-lang.org/cargo/)

---

## Long Prose

### Chapter 1 — The Beginning

It was a cold morning when the developer first opened their editor. The project had been sitting untouched for weeks, the code growing stale in the repository. There was something intimidating about coming back to a codebase after so long — like returning to a house you'd left in a hurry, not sure what you'd find inside.

The first thing to do was always `cargo check`. Not `cargo build`, never `cargo build` first. Just check. See what the compiler had to say. The compiler was always honest, sometimes brutally so, but you could trust it. It never lied.

The errors came back in red, as they always did. Twelve of them. Not bad for a month of neglect. Some were simple — unused imports, variables that had been renamed but references left behind. Others were more serious — a lifetime that had drifted out of scope, a trait implementation that no longer matched its interface.

### Chapter 2 — The Middle

Rust had a reputation for being difficult. And it was, at first. The borrow checker felt like an adversary, a strict professor marking your work with harsh red pen. Every time you thought you understood ownership, it would find a new way to confuse you.

But something changed after the first few months. The borrow checker stopped feeling like an enemy and started feeling like a colleague. A pedantic one, sure. One who would stop you mid-sentence to point out a logical inconsistency. But a trustworthy one. One who was almost always right.

The errors it caught were real errors. Memory bugs that in other languages would have silently corrupted data, or worse, compiled and shipped and caused problems in production at three in the morning. In Rust they were caught at compile time, before a single byte was written to disk.

### Chapter 3 — The Patterns

After a while, patterns emerged. The same solutions kept appearing for the same problems. Need to share data across threads? `Arc<Mutex<T>>`. Need to represent something that might not exist? `Option<T>`. Need to handle failure gracefully? `Result<T, E>`.

The standard library was surprisingly complete. Most things you needed were already there — collections, iterators, file I/O, networking primitives, threading utilities. And when the standard library wasn't enough, crates.io had almost certainly filled the gap.

The community had built extraordinary things. Async runtimes, web frameworks, game engines, embedded systems tools. The ecosystem was younger than some others but growing fast, and the quality was consistently high. People who chose Rust tended to care about correctness.

### Chapter 4 — Scroll Test

This section exists purely to make the document long enough to test scrolling behavior in the markdown reader. Scroll down and see how the content flows. Try adjusting the font size while scrolling. Try switching between markdown view and the raw text editor. Try opening a different file while this one is displayed.

The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.

Pack my box with five dozen liquor jugs. Pack my box with five dozen liquor jugs. Pack my box with five dozen liquor jugs. Pack my box with five dozen liquor jugs.

How vexingly quick daft zebras jump. How vexingly quick daft zebras jump. How vexingly quick daft zebras jump. How vexingly quick daft zebras jump.

Sphinx of black quartz, judge my vow. Sphinx of black quartz, judge my vow. Sphinx of black quartz, judge my vow. Sphinx of black quartz, judge my vow.

---

## Edge Cases

### Empty Sections

###

### Very Long Line

This is an extremely long line that goes on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and on and should test horizontal overflow or wrapping behavior.

### Special Characters

Ampersand: & Angle brackets: < > Quotes: " ' Backtick: \` Backslash: \\

### Numbers and Symbols

1234567890
!@#$%^&*()_+-=[]{}|;:,.<>?

---

*End of test document.*
