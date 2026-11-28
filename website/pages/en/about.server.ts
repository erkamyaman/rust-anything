import { defineHead } from 'void'

export const head = defineHead(() => ({
  title: 'About the author',
  meta: [
    {
      name: 'description',
      content:
        'Erkam Yaman is a software engineer in Istanbul, an Angular contributor and the founder of Rust Anything, a free tool that rewrites apps in Rust checked against the original.',
    },
  ],
}))
