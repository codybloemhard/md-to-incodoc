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

todo:

- emphasis
- super/sub script
- link
- image (as link)
- metadata block
  - convention for nav as metadata
- math
- html (ignore)
- soft and hard break (ignore)
- rule (ignore)

not yet supported in incodoc:

- block quote
- definition list
- table
- footnotes
