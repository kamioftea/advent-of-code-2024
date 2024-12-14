import highlightJs from 'highlight.js';
import markdown_it from 'markdown-it';

const md = markdown_it()

highlightJs.configure({})

const slugify = str => str?.replaceAll(/[^a-z0-9]+/gi, '-').toLocaleLowerCase()

export default function (string, language) {
    if (language && highlightJs.getLanguage(language)) {
        try {
            return `<pre class="hljs"><code class="code-block ${slugify(language)}">${
                highlightJs.highlight(string, {
                    language,
                    ignoreIllegals: true
                }).value
            }</code></pre>`;
        } catch (_) {}
    }

    return `<pre class="hljs"><code class="code-block ${slugify(language)}">${md.utils.escapeHtml(string)}</code></pre>`;
}
