import { useShared } from '@void/react'
import { useFrontmatter } from '@void/md'
import Navbar from '../../../components/site/Navbar' with { island: 'load' }
import Toc from '../../../components/docs/Toc' with { island: 'load' }
import Sidebar from '../../../components/docs/Sidebar'
import Breadcrumb from '../../../components/docs/Breadcrumb'
import Pager from '../../../components/docs/Pager'
import Footer from '../../../components/site/Footer'
import { EDIT_BASE } from '../../../lib/site'
import { messages, stripLocale } from '../../../lib/i18n'
import { localizedNav } from '../../../lib/nav'

const LOCALE = 'it'

export default function DocsLayout({ children }: { children: React.ReactNode }) {
  const shared = useShared<{ path?: string }>()
  const fm = useFrontmatter() as { source?: string }
  const currentPath = shared?.path ?? ''
  const editUrl = `${EDIT_BASE}/${fm.source ?? `website/pages/${LOCALE}${stripLocale(currentPath)}.md`}`
  const m = messages(LOCALE)

  return (
    <div className="min-h-screen bg-bg text-fg">
      <header className="sticky top-0 z-50 border-b border-line bg-bg/85 backdrop-blur-md">
        <Navbar currentPath={currentPath} locale={LOCALE} ui={m.ui} nav={localizedNav(LOCALE)} />
      </header>
      <div className="mx-auto flex w-full max-w-[1440px]">
        <aside className="hidden w-64 shrink-0 lg:block">
          <Sidebar currentPath={currentPath} locale={LOCALE} />
        </aside>
        <div className="min-w-0 flex-1 px-5 pt-8 pb-16 sm:px-8 lg:px-12">
          <div className="mx-auto max-w-3xl">
            <Breadcrumb currentPath={currentPath} locale={LOCALE} />
            <Toc variant="inline" ui={m.ui} />
            <main id="main-content" className="void-md" tabIndex={-1}>
              {children}
            </main>
            <a href={editUrl} className="mt-12 inline-block text-sm text-muted hover:text-fg xl:hidden">
              {m.ui.editPage} <span className="rtl-flip">→</span>
            </a>
            <Pager currentPath={currentPath} locale={LOCALE} />
          </div>
        </div>
        <aside className="hidden w-64 shrink-0 xl:block">
          <Toc variant="rail" editUrl={editUrl} ui={m.ui} />
        </aside>
      </div>
      <Footer locale={LOCALE} />
    </div>
  )
}
