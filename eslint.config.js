// @ts-check

import cspell from "@cspell/eslint-plugin"
import eslint from "@eslint/js"
import importHelpers from "eslint-plugin-import-helpers"
import prettierRecommended from "eslint-plugin-prettier/recommended"
import tsEslint from "typescript-eslint"

export default tsEslint.config(
  eslint.configs.recommended,
  // @ts-ignore:next-line
  prettierRecommended,
  ...tsEslint.configs.strictTypeChecked,
  ...tsEslint.configs.stylisticTypeChecked,
  {
    ignores: [
      "**/node_modules/**",
      "**/build/**",
      "**/dist/**",
      "**/scripts/**",
      "**/fixtures/**",
      "**/coverage/**",
      "**/__snapshots__/**",
      "commitlint.config.js",
      "ecosystem.config.cjs",
      "eslint.config.js"
    ]
  },
  {
    rules: {
      "@typescript-eslint/consistent-type-definitions": 0,
      "@typescript-eslint/require-await": 0,
      "@typescript-eslint/only-throw-error": 0,
      "@typescript-eslint/no-floating-promises": 0,
      "@typescript-eslint/no-extraneous-class": 0,
      "@typescript-eslint/no-misused-promises": 0,
      "@typescript-eslint/no-dynamic-delete": 0,
      "@typescript-eslint/no-non-null-assertion": 0,
      "@typescript-eslint/no-empty-function": 0,
      "@typescript-eslint/restrict-template-expressions": 0,
      "@typescript-eslint/return-await": 1,
      "@typescript-eslint/switch-exhaustiveness-check": 1,
      "@typescript-eslint/no-unused-expressions": 1,
      "@typescript-eslint/no-import-type-side-effects": 1,
      "@typescript-eslint/consistent-type-exports": 1,
      "@typescript-eslint/consistent-type-imports": 1
    }
  },
  {
    plugins: {
      "@typescript-eslint": tsEslint.plugin
    },
    languageOptions: {
      parser: tsEslint.parser,
      parserOptions: {
        project: "tsconfig.json"
      }
    }
  },
  {
    plugins: {
      importHelpers
    },
    rules: {
      "importHelpers/order-imports": [
        "warn",
        {
          newlinesBetween: "always",
          groups: ["module", "/^@root/", ["parent", "sibling", "index"]],
          alphabetize: {
            order: "asc",
            ignoreCase: true
          }
        }
      ]
    }
  },
  {
    plugins: {
      cspell
    },
    rules: {
      "cspell/spellchecker": [
        "warn",
        {
          customWordListFile: "./cspell-custom.txt"
        }
      ]
    }
  },
  {
    files: ["**/*.js"],
    extends: [tsEslint.configs.disableTypeChecked]
  }
)
