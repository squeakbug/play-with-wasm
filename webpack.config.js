const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode: "development",
  entry: './frontend/index.ts',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist'),
    clean: true,
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
  devServer: {
    static: path.resolve(__dirname, 'dist'),
    port: 3000,
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  experiments: {
    asyncWebAssembly: true
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './frontend/index.html',
      filename: 'index.html',
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, 'src'),
      watchDirectories: [
        path.resolve(__dirname, "./src")
      ],
    }),
  ]
};