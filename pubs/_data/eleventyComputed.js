import fs from 'fs/promises';
import path from 'node:path';

function injectWriteUpUrl(day, posts) {
    return posts[day] ? {'Write Up': posts[day]} : {};
}

async function buildDay(file, day, posts) {
    const contents = await fs.readFile(file, 'utf-8')
    const line = contents.split(/[\n\r]+/)[0]
    // This is my solution for [Advent of Code - Day 1: _Calorie
    // Counting_](https://adventofcode.com/2024/day/1)
    const [,title, puzzleURL] =
    line.match(/\[Advent of Code - Day \d+: _([^_]+)_]\(([^)]+)\)/) ?? []

    const links = {
        Puzzle: puzzleURL,
        ...(injectWriteUpUrl(day, posts)),
        Documentation: `/advent_of_code_2024/day_${day}/index.html`,
        Source: `https://github.com/kamioftea/advent-of-code-2024/blob/main/src/day_${day}.rs`
    }

    return {day, title, links};
}

async function buildSolutionData(posts) {
    const solutions = [];
    const dir = await fs.opendir(path.join('..', 'src'));
    for await (const entry of dir) {
        const matches = entry.name.match(/day_(\d+)\.rs/)
        if(entry.isFile() && matches) {
            solutions.push(await buildDay(path.join(dir.path, entry.name), parseInt(matches[1]), posts))
        }
    }
    return solutions
}


// noinspection JSUnusedGlobalSymbols
export default function () {
    // noinspection JSUnusedGlobalSymbols
    return {
        solutions: async (data) => {
            const postsCollection = data.collections.post;
            const posts = Object.fromEntries(
                [...(postsCollection ?? [])].map(post => [post.data.day, post.url])
            );
            return [...(await buildSolutionData(posts))].sort((a, b) => a.day - b.day)
        },
        title: data => data.title ||
            [
                data.header,
                'Advent of Code 2024',
                'Jeff Horton'
            ].join(' | '),
        description: data => {
            if(data.description) {
                return data.description
            }
            if(data.day && data.header) {
                return `A walkthrough of my solution for Advent of Code 2024 - ${data.header}`
            }
        }
    }
}
