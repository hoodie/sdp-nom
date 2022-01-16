const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: "sdp-wasm"
    })
  ],
  mode: "development",
  experiments: {
    //asyncWebAssembly: true // async don't work ATM : https://github.com/rustwasm/wasm-bindgen/issues/2343
    syncWebAssembly: true
  }
};
