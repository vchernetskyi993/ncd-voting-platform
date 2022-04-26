const webpack = require("webpack");

module.exports = function override(config, env) {
  config.resolve.fallback = {
    fs: false,
    path: false,
    util: false,
  };
  config.plugins.push(
    new webpack.ProvidePlugin({
      process: "process/browser",
      Buffer: ["buffer", "Buffer"],
    })
  );
  return config;
};
