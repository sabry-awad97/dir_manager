#! /usr/bin/env node

import figlet from 'figlet';

import { Command } from 'commander';

import * as fs from 'fs';
import * as path from 'path';

const program = new Command();

program
  .version('1.0.0')
  .description('An example CLI for managing a directory')
  .option('-l, --ls  [value]', 'List directory contents')
  .option('-m, --mkdir <value>', 'Create a directory')
  .option('-t, --touch <value>', 'Create a file')
  .parse(process.argv);

const options = program.opts<{
  ls: boolean | string;
  mkdir: string;
  touch: string;
}>();

console.log(figlet.textSync('Dir Manager'));

async function listDirContents(filepath: string) {
  try {
    const files = await fs.promises.readdir(filepath);
    const detailedFilesPromises = files.map(async (file: string) => {
      const { size, birthtime } = await fs.promises.lstat(
        path.resolve(filepath, file)
      );
      return {
        filename: file,
        // size in KB
        size: size,
        createdAt: birthtime,
      };
    });

    const detailedFiles = await Promise.all(detailedFilesPromises);
    console.table(detailedFiles);
  } catch (error) {
    console.error('Error occurred while reading the directory!', error);
  }
}

function createDir(filepath: string) {
  if (!fs.existsSync(filepath)) {
    fs.mkdirSync(filepath);
    console.log('The directory has been created successfully');
  }
}

function createFile(filepath: string) {
  fs.openSync(filepath, 'w');
  console.log('An empty file has been created');
}

if (options.ls) {
  const filepath = typeof options.ls === 'string' ? options.ls : __dirname;
  listDirContents(filepath);
}

if (options.mkdir) {
  createDir(path.resolve(__dirname, options.mkdir));
}

if (options.touch) {
  createFile(path.resolve(__dirname, options.touch));
}

if (!process.argv.slice(2).length) {
  program.outputHelp();
}
