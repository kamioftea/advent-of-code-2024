import {EleventyRenderPlugin} from '@11ty/eleventy';

import inclusiveLangPlugin from '@11ty/eleventy-plugin-inclusive-language';

import markdown from './_lib/markdown-it.js';

// noinspection JSUnusedGlobalSymbols
export default function (eleventyConfig) {
    eleventyConfig.addPlugin(EleventyRenderPlugin)
    eleventyConfig.addPlugin(inclusiveLangPlugin);

    eleventyConfig.setLibrary('md', markdown())

    // IntelliJ doesn't like frontmatter before <!doctype html> in root layout
    // So add the layout defaults here
    eleventyConfig.addGlobalData('layout', 'layout.njk')

    eleventyConfig.addPassthroughCopy('assets')

    return {
        passthroughFileCopy:    true,
        markdownTemplateEngine: 'njk',
        pathPrefix:             process.env.PATH_PREFIX ?? ''
    }
}
