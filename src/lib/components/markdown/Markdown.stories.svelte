<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import Markdown from "./Markdown.svelte";

  const { Story } = defineMeta({
    title: "Custom/Markdown",
    component: Markdown,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Headings"
  args={{
    source: `
# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6
`,
  }}
/>

<Story
  name="Text Styles"
  args={{
    source: `
**Bold** text, *italic* text, _underline_, ~~strikethrough~~, and ***combined***.
> A blockquote spanning multiple lines to demonstrate quotes.
[Example link](https://example.com)
![Alt text image](https://via.placeholder.com/150)
`,
  }}
/>

<Story
  name="Lists"
  args={{
    source: `
- First bullet
- Second bullet
  - Nested bullet
    - Deeper level

1. First numbered
2. Second numbered
   1. Sub numbered
   2. Another sub
`,
  }}
/>

<Story
  name="Code Snippets"
  args={{
    source: `
Here is \`inline code\`.

\`\`\`js
// JavaScript example
const greet = name => \`Hello, \${name}!\`;
console.log(greet("Storybook"));
\`\`\`

\`\`\`python
# Python example
def greet(name):
    return f"Hello, {name}"
print(greet("Storybook"))
\`\`\`
`,
  }}
/>

<Story
  name="Tables"
  args={{
    source: `
| Item     | Description      | Price |
|----------|------------------|------:|
| Apple    | Fresh red apple  | $1.20 |
| Banana   | Ripe yellow banana | $0.80 |
| Cherry   | Sweet cherries   | $2.00 |

End of table
`,
  }}
/>

<Story
  name="Horizontal Rules & Line Breaks"
  args={{
    source: `
First line  
Second line after two spaces

---

Another paragraph separated by a horizontal rule
`,
  }}
/>

<Story
  name="Mixed Features"
  args={{
    source: `
# Mixed Demo

This is a **bold** introduction with a [link](https://storybook.js.org), followed by a list:

- Item 1
- Item 2
  1. Sub 1
  2. Sub 2

\`\`\`bash
echo "Shell script code block"
\`\`\`

> A sample blockquote.

| Col A | Col B |
|-------|-------|
| A1    | B1    |
| A2    | B2    |

End of mixed demo.
`,
  }}
/>

<Story
  name="Emoji & HTML Tags"
  args={{
    source: `
Here is emoji 🎉

<div style="color: red;">This is red text using inline HTML</div>

<img src="https://via.placeholder.com/100" alt="Placeholder image">
`,
  }}
/>
