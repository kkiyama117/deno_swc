import init, {
  parseSync,
  spanCommentsSync,
  printSync,
  source,
  transformSync,
} from "./swc_wasm/wasm.js";

import {
  Config,
  Comment,
  ParseOptions,
  Program,
  TransformConfig,
} from "./types/options.ts";

await init(source);

export function parse(source: string, opts: ParseOptions): Program {
  return parseSync(source, opts);
}

export function spanComments(source: string, opts: ParseOptions): [ Comment ] {
  return spanCommentsSync(source, opts);
}

// TODO(littledivy): Typings for `program`
export function print(program: any, opts?: Config): { code: string } {
  return printSync(program, opts || {});
}

export function transform(
  source: string,
  opts: TransformConfig,
): { code: string } {
  return transformSync(source, opts);
}
