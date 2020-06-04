module.exports = {
	watch: true,
	entry: './src/index.jsx',
	output: {
		filename: '../wwwroot/dist/main.js'
	},
	module: {
		rules: [{
				test: /\.jsx$/,
				exclude: /node_modules/,
				use: {
					loader: "babel-loader"
				}
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