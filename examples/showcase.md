# Agent EPUB feature showcase

A small reading edition that shows what Agent EPUB produces from ordinary
Markdown. The cover, contents page, reader contents menu and chapter navigation
are generated for you. This introduction stays before the H2 chapters because
the accompanying request selects `"split_level": 2`.

Try the book's contents menu, enlarge the text, follow a note and use its return
link. The illustrations are synthetic, and the table values are examples rather
than research findings[^sample].

## Text, structure and navigation

### Typography and Unicode

Ordinary paragraphs can contain **strong emphasis**, *emphasis*, and
`inline code`. Characters such as café, naïef, Ελληνικά and 日本語 remain text.
An ampersand & and angle brackets in code, `<chapter>`, are preserved safely.

This is a reflowable book: readers choose the font and text size. It is useful
for reports, guides and reading notes whose layout should adapt to the screen.

#### Line breaks

These two lines use a deliberate hard break.  
The second line stays on its own line within the paragraph.

This is a separate paragraph, with a normal paragraph gap.

### Lists and quotations

An ordered workflow:

1. Finish the writing and collect the local illustrations.
2. Submit the source files or a version 1 JSON request.
3. Inspect the result and open the EPUB in a reader.

A nested list:

- Reading content
  - Chapters and sections
  - Tables, images and notes
- Generated navigation
  - Visible contents and reader contents
  - Previous/next chapter links and return-to-contents links

> A reading edition should preserve the writing and let the reader choose
> how comfortably to read it.
>
> This second quoted paragraph belongs to the same quotation.

### Code and literal characters

Fenced code preserves whitespace and punctuation. A language label does not
add syntax highlighting.

```python
def reading_time(words, words_per_minute=200):
    return words / words_per_minute

print(reading_time(1200))
# Literal characters: < > & " ' $ ~
```

---

The horizontal rule above separates two parts of this chapter.

### Deeper contents

The contents menu includes sections as nested entries. The following heading
deliberately skips H4; it attaches beneath the nearest shallower section.

##### A skipped level

This H5 heading keeps its original rank in the reading content.

###### One final level

This H6 heading demonstrates the deepest supported contents level.

### Links and chapter boundaries

Follow [the illustrated chapter](#tables-images-and-notes), jump to its
[PNG diagram](#a-png-diagram), or open the supplied
[literal-text appendix](Literal%20text%20appendix.txt).
The appendix also demonstrates linking between supplied source files.

An [external example link](https://example.org/) and an illustrative
[email link](mailto:reader@example.org) remain external; generation does not
fetch websites or send mail. A [reference-style link][example-site] works too.

The introductory note can be reached from this chapter as well[^sample].

## Tables, images and notes

### A simple table

This rectangular pipe table has a header row and left, right and centered
alignment. Headers retain their column relationships in the EPUB.

| Feature | Example count | Status |
| :--- | ---: | :---: |
| Chapters from this Markdown file | 3 | Included |
| Unique inline image files | 3 | Included |
| Literal pipe in code: `alpha \| beta` | 1 | Preserved |

These are explanatory example values, not measured performance results.

### A PNG diagram

![Three colored stages connected from left to right, representing writing, packaging and reading](assets/export-flow.png "From writing to reading")

The PNG has supplied alternative text and an optional title. It shrinks to the
reading area without changing its proportions. The original image bytes are
bundled in the book[^diagram].

### An image inside a table

The same local image is reused here; it does not become another inline asset.

| Resource | Preview |
| :--- | :---: |
| Reused workflow diagram | ![The same three-stage writing, packaging and reading diagram](assets/export-flow.png) |

### A JPEG illustration

![A synthetic blue and purple sky above a green landscape](assets/landscape.jpg "Synthetic landscape")

This JPEG is also used as the supplied cover. The cover is identified separately
from chapter illustrations and is excluded from the chapter count.

### An explicit decorative image

The divider below conveys no information beyond separating the text. Empty
brackets and the exact title `"decorative"` request empty alternative text and
a presentation role.

![](assets/divider.png "decorative")

### Shared notes and return links

This chapter refers to the same introductory note[^sample]. It also refers
again to the diagram note[^diagram]. Follow either reference to its note and
choose the return link for the place you came from. A note appears at the end
of the first chapter that refers to it, even when a later chapter reuses it.

Return to [typography and Unicode](#typography-and-unicode) or continue to the
[literal-text appendix](Literal%20text%20appendix.txt).

[^sample]: All writing, illustrations and table values in this showcase are synthetic examples.

    Notes can contain more than one paragraph, **emphasis**, `code` and a
    [link to another chapter](#tables-images-and-notes). Repeated references
    share this one definition and receive separate return links.

[^diagram]: Notes can also include local images. Here is the reused diagram:

    ![The three-stage workflow diagram repeated in a supporting note](assets/export-flow.png)

[example-site]: https://example.org/
