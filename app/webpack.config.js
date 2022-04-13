module.exports = {
	watch: false,
	entry: "./src/index.tsx",
	output: {
		filename: "../wwwroot/dist/main.js"
	},
	resolve: {
		extensions: [".js", ".ts", ".tsx"]
	},
	module: {
		rules: [{
				test: /\.tsx$/,
				exclude: /node_modules/,
				use: ["babel-loader", "ts-loader"]
			},
			{
				test: /\.ts$/,
				exclude: /node_modules/,
				use: ["ts-loader"]
			},
			{
				test: /\.scss$/,
				use: ["style-loader", "css-loader", "sass-loader"]
			},
			{
				test: /\.css$/,
				use: ["style-loader", "css-loader"]
			},
			{
				test: /\.js$/,
				use: ["@open-wc/webpack-import-meta-loader"],
			}
		]
	}
};