# md-to-incodoc

Convert markdown to incodoc.

very much work in progress.

converted:

- paragraph -> paragraph
- heading -> section with headings
- lists
  - unordered -> identical
  - ordered -> distinct
  - checked -> checked
- code
  - inline -> text tagged "code"
  - block -> codeblock
- emphasis
  - *emphasis* -> light emphasis
  - **strong** -> medium emphasis
  - ***emphasis strong*** -> strong emphasis
  - ~strikethrough~ -> medium deemphasis
- soft/hard breaks and rule -> soft break
- super/sub script -> text/emphasis with tags "super" and "sub"
- link -> link, corporeal link types discarded
- image -> link tagged as image
- html
  - html block -> code block tagged "unconv-corp" (unconverted corporeal content)
  - inline html -> inside "html()" enclosure, text with html tags stripped
- math
  - inline -> text tagged "latex-math"
  - display -> code block with language set "latex-math" and mode set "replace"

todo:

- metadata block
  - convention for nav as metadata

not yet supported in incodoc:

- block quote
- definition list
- table
- footnotes
