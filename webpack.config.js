const webpack = require("webpack");
const path = require('path');
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  mode: "development",
  entry: './frontend/index.ts',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist'),
  },
  module: {
    rules: [
      {
        test: /fibonacci\.wasm$/,
        type: "javascript/auto",
        loader: "file-loader",
      },
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
    ]
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  experiments: {
    asyncWebAssembly: true
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: "frontend", to: "." },
      ],
    }),
  ]
};