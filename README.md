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
  - *emphasis* => light emphasis
  - **strong** => medium emphasis
  - ***emphasis strong*** => strong emphasis
  - ~strikethrough~ => medium deemphasis
- soft and hard breaks => soft break

todo:

- super/sub script
- link
- image (as link)
- metadata block
  - convention for nav as metadata
- math
- html (ignore)
- rule (ignore)

not yet supported in incodoc:

- block quote
- definition list
- table
- footnotes
