import { it, expect, describe } from "vitest";

import { replaceMarkdownRelativeUrls } from "./markdown";

describe("replaceMarkdownRelativeUrls", () => {
  const baseUrl = "https://example.com/docs/";

  it("should replace relative markdown links with absolute URLs", () => {
    const md = "Some text with [link](./page.md)";
    const result = replaceMarkdownRelativeUrls(md, baseUrl);
    expect(result).toBe(
      "Some text with [link](https://example.com/docs/page.md)",
    );
  });

  it("should replace relative image links with absolute URLs", () => {
    const md = "![alt](images/pic.png)";
    const result = replaceMarkdownRelativeUrls(md, baseUrl);
    expect(result).toBe("![alt](https://example.com/docs/images/pic.png)");
  });

  it("should leave absolute markdown links unchanged", () => {
    const md = "Go to [Google](https://google.com)";
    const result = replaceMarkdownRelativeUrls(md, baseUrl);
    expect(result).toBe(md);
  });

  it("should replace HTML src attributes with absolute URLs", () => {
    const md = '<img src="./img/logo.png" />';
    const result = replaceMarkdownRelativeUrls(md, baseUrl);
    expect(result).toBe('<img src="https://example.com/docs/img/logo.png" />');
  });

  it("should replace HTML href attributes with absolute URLs", () => {
    const md = '<a href="guide/start.html">Start</a>';
    const result = replaceMarkdownRelativeUrls(md, baseUrl);
    expect(result).toBe(
      '<a href="https://example.com/docs/guide/start.html">Start</a>',
    );
  });

  it("should not modify data URLs, hash links, or mailto links", () => {
    const md = `
![inline](data:image/png;base64,xxx)
[hash](#section)
[email](mailto:test@example.com)
`;
    const result = replaceMarkdownRelativeUrls(md, baseUrl);
    expect(result).toContain("![inline](data:image/png;base64,xxx)");
    expect(result).toContain("[hash](#section)");
    expect(result).toContain("[email](mailto:test@example.com)");
  });

  it("should handle baseUrl with trailing slashes correctly", () => {
    const md = "[link](file.md)";
    const result = replaceMarkdownRelativeUrls(
      md,
      "https://example.com/docs////",
    );
    expect(result).toBe("[link](https://example.com/docs/file.md)");
  });

  it("should handle relative paths with ./ and / correctly", () => {
    const md = `
[dot-slash](./foo/bar.md)
[leading-slash](/foo/bar.md)
[no-dot](foo/bar.md)
`;
    const result = replaceMarkdownRelativeUrls(md, baseUrl);
    expect(result).toContain(
      "[dot-slash](https://example.com/docs/foo/bar.md)",
    );
    expect(result).toContain(
      "[leading-slash](https://example.com/docs/foo/bar.md)",
    );
    expect(result).toContain("[no-dot](https://example.com/docs/foo/bar.md)");
  });
});
