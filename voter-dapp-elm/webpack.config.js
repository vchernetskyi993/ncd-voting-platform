const Dotenv = require("dotenv-webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const HtmlWebpackElementPlugin = require("html-webpack-element-plugin");

module.exports = {
  entry: "./src/index.ts",
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.elm$/,
        exclude: [/elm-stuff/, /node_modules/],
        use: [
          { loader: "elm-hot-webpack-loader" },
          {
            loader: "elm-webpack-loader",
            options: {},
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: [".js", ".ts", ".elm"],
    fallback: {
      fs: false,
      path: false,
      util: false,
    },
  },
  plugins: [
    new Dotenv(),
    new HtmlWebpackPlugin(),
    new HtmlWebpackElementPlugin(),
  ],
};
