import { AsyncSeriesWaterfallHook } from "tapable";
import { Compiler, Compilation } from "webpack";

export = HtmlWebpackPlugin;

declare class HtmlWebpackPlugin {
	constructor(searchPath: string);

	searchPath: string;
	apply(compiler: Compiler): void;
}
