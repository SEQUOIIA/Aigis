var webpack = require("webpack");
var path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const VueLoaderPlugin = require('vue-loader/lib/plugin')
const CopyWebpackPlugin = require('copy-webpack-plugin')

const rendererConfig = (env, argv) => {
    return {
        entry: {
            RendererMain: ["./src/renderer/main.ts"],
        },
        output: {
            filename: "[name]." + argv.mode + ".js",
            path: __dirname + "/dist",
            libraryTarget: "umd",
            library: ["AIGIS", "[name]"]
        },

        devServer: {
            contentBase: path.join(__dirname, 'dist'),
            compress: true,
            port: 9000
        },

        target: 'electron-renderer',
        devtool: "source-map",
    
        resolve: {
            extensions: [".webpack.js", ".web.js", ".ts", ".tsx", ".js", ".jsx", ".scss", ".vue"],
            alias: {
                'vue$': 'vue/dist/vue.esm.js'
            }
        },
    
        module: {
            rules: [
                { test: /\.tsx?$/, exclude: /node_modules/, use:[
                    {
                        loader: "babel-loader",
                        options: {
                            presets: ['@babel/preset-env', 
                                {
                                }
                            ], 
                            sourceMaps: false
                        }
                    },
                    {
                        loader: "ts-loader",
                        options: {
                            appendTsSuffixTo: [/\.vue$/],
                            appendTsxSuffixTo: [/\.vue$/]
                        }
                    }
                ] },

                { test: /\.js?$/, exclude: /node_modules/, use:[
                    {
                        loader: "babel-loader"
                    }
                ] },

                { test: /\.vue?$/, exclude: /node_modules/, use:[
                    {
                        loader: "vue-loader"
                    }
                ] },
            

                { test: /\.scss?$/, exclude: /node_modules/, use: [
                    {
                        loader: MiniCssExtractPlugin.loader,
                    },
                    'css-loader',
                    'sass-loader',
                ]},

                {
                    test: /\.(gif|svg|jpg|png|woff|woff2)$/,
                    loader: "file-loader",
                },
            ]
        },
    
        plugins: [
            new MiniCssExtractPlugin({
                filename: "[name]." + argv.mode + ".css",
                chunkFilename: "[id].css"
            }),
            new HtmlWebpackPlugin({
                template: 'index.html'
            }),
            new VueLoaderPlugin(),
            new CopyWebpackPlugin([
                {from: '../static', to: 'static'}
            ]),
        ]
    }
}

const mainConfig = (env, argv) => {
    return {
        entry: {
            Main: ["./src/main/main.ts"],
        },
        output: {
            filename: "[name]." + argv.mode + ".js",
            path: __dirname + "/dist",
        },

        target: 'electron-main',
        devtool: "source-map",
    
        resolve: {
            extensions: [".webpack.js", ".web.js", ".ts", ".tsx", ".js", ".jsx", ".scss"]
        },
    
        module: {
            rules: [
                { test: /\.tsx?$/, exclude: /node_modules/, use:[
                    {
                        loader: "babel-loader",
                        options: {
                            presets: ["@babel/preset-env", 
                                {
                                }
                            ], 
                            sourceMaps: false
                        }
                    },
                    {
                        loader: "ts-loader"
                    }
                ] },
            ]
        },
    }
}

module.exports = [mainConfig, rendererConfig];
