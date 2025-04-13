/**
 * Replaces all relative URLs in a markdown file content
 * with absolute URLs using the provided base URL
 *
 * @param markdown
 * @param baseUrl
 */
export function replaceMarkdownRelativeUrls(markdown: string, baseUrl: string) {
  if (baseUrl === null) return markdown;
  baseUrl = baseUrl.replace(/\/+$/, "") + "/";

  // Markdown style [text](url) and ![alt](url)
  markdown = markdown.replace(
    /(!?\[[^\]]*?\]\()([^)]+)(\))/g,
    (match, prefix, url, suffix) => {
      if (isAbsoluteUrl(url)) return match;
      const resolved = resolveUrl(baseUrl, url);
      return `${prefix}${resolved}${suffix}`;
    },
  );

  // HTML style src="..." and href="..."
  markdown = markdown.replace(
    /(src|href)\s*=\s*"([^"]+)"/gi,
    (match, attr, url) => {
      if (isAbsoluteUrl(url)) return match;
      const resolved = resolveUrl(baseUrl, url);
      return `${attr}="${resolved}"`;
    },
  );

  return markdown;
}

function isAbsoluteUrl(url: string): boolean {
  return (
    /^(?:[a-z]+:)?\/\//i.test(url) ||
    url.startsWith("data:") ||
    url.startsWith("#") ||
    url.startsWith("mailto:")
  );
}

function resolveUrl(base: string, relative: string): string {
  return base + relative.replace(/^\.?\/+/, "");
}
