module.exports = {
  env: {
    node: true,
    es2021: true,
    mocha: true,
  },
  extends: ["standard", "prettier"],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaVersion: "latest",
    sourceType: "module",
  },
  plugins: ["@typescript-eslint", "prettier"],
  rules: {
    "prettier/prettier": ["warn"],
    "@typescript-eslint/explicit-function-return-type": "error",
  },
};
