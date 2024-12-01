import highlightJs from 'highlight.js';
import markdown_it from 'markdown-it';

const  md = markdown_it()

highlightJs.configure({})

export default function (string, language) {
    if (language && highlightJs.getLanguage(language)) {
        try {
            return '<pre class="hljs"><code class="code-block">' +
                highlightJs.highlight(string, { language, ignoreIllegals: true }).value +
                '</code></pre>';
        } catch (__) {}
    }

    return '<pre class="hljs"><code class="code-block">' + md.utils.escapeHtml(string) + '</code></pre>';
}
