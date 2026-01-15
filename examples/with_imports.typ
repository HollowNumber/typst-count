= Document with Imports

This is the main document that imports content from other files.

#import "shared.typ": *

== Main Content

This document demonstrates how typst-count handles imported content.

#include "chapter1.typ"

#include "chapter2.typ"

== Conclusion

By default, typst-count includes all imported and included content in the word count.
Use the `--exclude-imports` flag to count only this main file.