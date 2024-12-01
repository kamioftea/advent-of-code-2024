import highlight from './highlight.js';
import mathjax from 'markdown-it-mathjax';
import defList from 'markdown-it-deflist';
import anchor from 'markdown-it-anchor';
import MarkdownIt from 'markdown-it';

const defaultOpts = {
    baseOpts:          {},
    headingPermalinks: true,
}

export default (options = defaultOpts) => {
    const opts = {
        breaks:      false,
        highlight,
        html:        true,
        linkify:     false,
        typographer: true,

        ...(options.baseOpts)
    }


    return new MarkdownIt(opts)
        .use(anchor, {
            permalink: options.headingPermalinks ?
                           anchor.permalink.headerLink({
                               class:           'app-link--heading',
                               safariReaderFix: true
                           }) :
                           false
        })
        .use(defList)
        .use(mathjax());
}
