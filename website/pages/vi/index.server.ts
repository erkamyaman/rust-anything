import { defineHead } from 'void'
import { messages } from '../../lib/i18n.ts'

const m = messages('vi').meta

export const head = defineHead(() => ({
  title: m.title,
  meta: [{ name: 'description', content: m.description }],
}))
