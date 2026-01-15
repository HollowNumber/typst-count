// Shared definitions and functions

#let highlight(content) = {
  text(fill: blue, content)
}

#let note(content) = {
  block(
    fill: luma(230),
    inset: 8pt,
    radius: 4pt,
    [*Note:* #content]
  )
}

#let author = "Example Author"
#let date = datetime.today()