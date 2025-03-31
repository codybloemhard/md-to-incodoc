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
- code and codeblock -> codeblock
- emphasis
  - *emphasis* -> light emphasis
  - **strong** -> medium emphasis
  - ***emphasis strong*** -> strong emphasis
  - ~strikethrough~ -> medium deemphasis
- soft/hard breaks and rule -> soft break
- super/sub script -> text/emphasis with tags "super" and "sub"
- link -> link, corporeal link types discarded

todo:

- image (as link)
- metadata block
  - convention for nav as metadata
- math
- html (ignore or codeblock?)

not yet supported in incodoc:

- block quote
- definition list
- table
- footnotes
