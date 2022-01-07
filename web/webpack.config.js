const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebPackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = (env, args) => {
    const isProductionMode = (args.mode === 'production');
    return {
        entry: './index.js',
        experiments: {
            asyncWebAssembly: true
        },
        output: {
            path: path.resolve(__dirname, 'dist'),
            filename: isProductionMode ? '[name].[contenthash].js' : '[name].[hash].js',
        },
        plugins: [
            new HtmlWebPackPlugin({
                template: 'index.html'
            }),
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, '.'),
            }),
            new webpack.ProvidePlugin({
                TextDecoder: ['text-encoding', 'TextDecoder'],
                TextEncoder: ['text-encoding', 'TextEncoder']
            })
        ]


    }
}