import {EleventyRenderPlugin} from '@11ty/eleventy';

import inclusiveLangPlugin from '@11ty/eleventy-plugin-inclusive-language';

import markdown from './_lib/markdown-it.js';

export default function (eleventyConfig) {
    eleventyConfig.addPlugin(EleventyRenderPlugin)
    eleventyConfig.addPlugin(inclusiveLangPlugin);

    eleventyConfig.setLibrary('md', markdown())

    // IntelliJ doesn't like frontmatter before <!doctype html> in root layout
    // So add the layout defaults here
    eleventyConfig.addGlobalData('layout', 'layout.njk')

    eleventyConfig.addPassthroughCopy('assets')
    eleventyConfig.addPassthroughCopy({'node_modules/highlight.js/styles/a11y-dark.css': 'assets/styles/a11y-dark.css'})

    return {
        passthroughFileCopy: true,
        markdownTemplateEngine: 'njk',
        pathPrefix: process.env.PATH_PREFIX ?? ''
    }
}
