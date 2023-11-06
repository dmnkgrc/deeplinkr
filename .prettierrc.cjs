// @ts-check

/** @type {import("@ianvs/prettier-plugin-sort-imports").PrettierConfig} */
module.exports = {
    // Standard prettier options
    singleQuote: true,
    semi: true,
    // Since prettier 3.0, manually specifying plugins is required
    plugins: ['prettier-plugin-svelte', '@ianvs/prettier-plugin-sort-imports', 'prettier-plugin-tailwindcss'],
    // This plugin's options
    importOrderParserPlugins: ['typescript', 'jsx', 'decorators-legacy'],
    importOrderTypeScriptVersion: '5.0.0',
};
