import { readFileSync, writeFileSync, readdirSync, statSync } from 'fs';
import { join } from 'path';

function fixPathsInFile(filePath) {
    let content = readFileSync(filePath, 'utf-8');
    const modified = content.replace(/(['"])\/(_app\/[^'"]+)(['"])/g, '$1./$2$3');
    if (content !== modified) {
        writeFileSync(filePath, modified, 'utf-8');
        console.log(`Fixed paths in: ${filePath}`);
    }
}

function processDirectory(dir) {
    const entries = readdirSync(dir);
    for (const entry of entries) {
        const fullPath = join(dir, entry);
        const stat = statSync(fullPath);
        if (stat.isDirectory()) {
            processDirectory(fullPath);
        } else if (entry.endsWith('.html') || entry.endsWith('.js')) {
            fixPathsInFile(fullPath);
        }
    }
}

processDirectory('build');
console.log('Path fixing complete!');
