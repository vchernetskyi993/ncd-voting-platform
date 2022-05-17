const HtmlWebpackPlugin = require('html-webpack-plugin');
const HtmlWebpackElementPlugin = require('html-webpack-element-plugin');

module.exports = {
    module: {
        rules: [
            {
                test: /\.elm$/,
                exclude: [/elm-stuff/, /node_modules/],
                use: {
                    loader: 'elm-webpack-loader',
                    options: {}
                }
            }]
    },
    plugins: [
        new HtmlWebpackPlugin(),
        new HtmlWebpackElementPlugin(),
    ]
}
