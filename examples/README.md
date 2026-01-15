# Examples

This directory contains example Typst documents for testing and demonstrating typst-count functionality.

## Files

### simple.typ
A basic Typst document with common formatting elements:
- Headings
- Body text
- Lists (bulleted and numbered)
- Bold and italic text
- Multiple sections

Expected count: approximately 80-90 words

Usage:
```bash
typst-count examples/simple.typ
```

### with_imports.typ
A main document that imports and includes content from other files. This demonstrates how typst-count handles multi-file projects.

Related files:
- `shared.typ` - Shared functions and definitions
- `chapter1.typ` - First chapter content
- `chapter2.typ` - Second chapter content

Expected count (with imports): approximately 450-500 words
Expected count (without imports): approximately 30-40 words

Usage:
```bash
# Count all content (default)
typst-count examples/with_imports.typ

# Count only the main file
typst-count examples/with_imports.typ --exclude-imports
```

### shared.typ
Contains reusable function definitions and variables. This file is imported by `with_imports.typ` but contains minimal text content (only comments and function definitions).

### chapter1.typ
A standalone chapter about Typst basics. Can be compiled independently or included in other documents.

Expected count: approximately 150-170 words

### chapter2.typ
A standalone chapter about advanced Typst features. Can be compiled independently or included in other documents.

Expected count: approximately 250-270 words

## Testing Different Scenarios

### Basic counting
```bash
typst-count examples/simple.typ
```

### Multiple files
```bash
typst-count examples/simple.typ examples/chapter1.typ examples/chapter2.typ
```

### JSON output
```bash
typst-count examples/simple.typ --format json
```

### CSV output
```bash
typst-count examples/*.typ --format csv --output counts.csv
```

### Exclude imports
```bash
typst-count examples/with_imports.typ --exclude-imports
```

### Word count only
```bash
typst-count examples/simple.typ --words
```

### Character count only
```bash
typst-count examples/simple.typ --characters
```

### Limit checking
```bash
# Should succeed
typst-count examples/simple.typ --min-words 50 --max-words 150

# Should fail (word count too low)
typst-count examples/simple.typ --min-words 200
```

## Creating Your Own Examples

When creating test documents:
1. Use realistic content that demonstrates typical use cases
2. Include various Typst features (headings, lists, emphasis, etc.)
3. Document the expected word/character counts
4. Test both simple and complex scenarios (imports, includes, etc.)
5. Keep examples focused and not too large