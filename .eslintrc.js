module.exports = {
  extends: 'erb/typescript',
  rules: {
    'import/no-extraneous-dependencies': 'off',
    'global-require': 'off',
    'no-console': 'off',
    '@typescript-eslint/no-use-before-define': 'off',
    'react/prefer-stateless-function': 'off',
    'no-shadow': 'off',
    'react/jsx-props-no-spreading': 'off',
    'react/sort-comp': 'off',
    'promise/always-return': 'off'
  },
  settings: {
    'import/resolver': {
      node: {},
      webpack: {
        config: require.resolve('./configs/webpack.config.eslint.js')
      }
    }
  }
};
