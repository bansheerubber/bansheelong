import WatchExternalFilesPlugin from "webpack-watch-files-plugin";
import * as fs from "fs";
import * as path from "path";
import * as webpack from 'webpack';

class HTMLLoader {
	searchPath: string;

	constructor(searchPath: string) {
		this.searchPath = searchPath;
	}
	
	apply(compiler: webpack.Compiler) {
		// Specify the event hook to attach to
		compiler.hooks.thisCompilation.tap(
			"HTMLLoader",
			(compilation) => {
				compilation.hooks.processAssets.tapAsync(
					{
						name: "HTMLLoader",
						stage: webpack.Compilation.PROCESS_ASSETS_STAGE_OPTIMIZE_INLINE,
					},
					(_, callback) => {
						const recurse = (directoryName: string) => {
							for (const file of fs.readdirSync(directoryName)) {
								const fullFile = `${directoryName}/${file}`;
								const stat = fs.statSync(fullFile);
								if (stat.isDirectory()) {
									recurse(fullFile);
								} else {
									const base = fullFile.replace(`${this.searchPath}/`, "");
									compilation.emitAsset(base, new webpack.sources.RawSource(fs.readFileSync(fullFile), true));
								}
							}
						};

						recurse(this.searchPath);

						callback();
					}
				)
			}
		);
	}
}

const targets: { [key: string]: webpack.Configuration } = {
	"release": {
		entry: "./src/index.ts",
		mode: "production",
		module: {
			rules: [{
				test: /\.tsx?$/,
				use: "ts-loader",
				exclude: /node_modules/,
			}, {
				test: /\.s?css$/i,
				use: [
					"style-loader",
					"css-loader",
					"sass-loader",
				],
			}, {
				test: /\.html$/i,
				loader: path.resolve(__dirname, "html-loader.js"),
			}],
		},
		plugins: [new HTMLLoader(path.resolve(__dirname, "src/html"))],
		output: {
			filename: "bundle.min.js",
		},
		resolve: {
			extensions: [".tsx", ".ts", ".js"],
		},
	},
	"debug": {
		devtool: "inline-source-map",
		entry: "./src/index.ts",
		mode: "development",
		module: {
			rules: [{
				test: /\.tsx?$/,
				use: "ts-loader",
				exclude: /node_modules/,
			}, {
				test: /\.s?css$/i,
				use: [
					"style-loader",
					"css-loader",
					"sass-loader",
				],
			}],
		},
		plugins: [
			new HTMLLoader(path.resolve(__dirname, "src/html")),
			new WatchExternalFilesPlugin({
				files: [
					"./src/html/**/*.html",
				],
			}),
		],
		output: {
			filename: "bundle.min.js",
		},
		resolve: {
			extensions: [".tsx", ".ts", ".js"],
		},
	},
}

module.exports = (env: any, argv: string[]) => {
	/** @type {string | null} */
	let targetName = Object.getOwnPropertyNames(targets)
		.reduce((prev: string | null, current: string) => prev ?? (env[current] ? current : null), null);
	
	if (targetName === null) {
		console.warn("Could not find specified target, defaulting to 'debug'");
	}

	targetName ??= "debug";
	
	const target = targets[targetName]!;
	const outputPath = path.resolve(__dirname, `target/${targetName}`);
	target.output!.path = outputPath;

	try {
		fs.rmSync(outputPath, { recursive: true });
	} catch {
		console.warn("Could not clean target directory")
	}

	return target;
};
