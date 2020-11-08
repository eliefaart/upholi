module.exports = {
	watch: false,
	entry: './src/index.jsx',
	output: {
		filename: '../wwwroot/dist/main.js'
	},
	module: {
		rules: [
			{
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
				test: /\.jsx$/,
				exclude: /node_modules/,
				use: [ "babel-loader" ]
			},
			{
				test: /\.scss$/,
				use: ['style-loader', 'css-loader', 'sass-loader']
			},
			{
				test: /\.css$/,
				use: ['style-loader', 'css-loader']
			}
		]
	}
};