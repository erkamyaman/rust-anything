import Navbar from '../../components/site/Navbar' with { island: 'load' }
import Footer from '../../components/site/Footer'
import { messages } from '../../lib/i18n'
import { localizedNav } from '../../lib/nav'
import { REPO_URL } from '../../lib/site'

const LOCALE = 'en'
const REAL_WORLD = `${REPO_URL}/blob/main/docs/real-world.md`

export default function About() {
  const m = messages(LOCALE)
  return (
    <div className="min-h-screen bg-bg text-fg">
      <header className="sticky top-0 z-50 border-b border-line bg-bg/85 backdrop-blur-md">
        <Navbar currentPath="/about" locale={LOCALE} ui={m.ui} nav={localizedNav(LOCALE)} />
      </header>
      <div className="px-5 pt-12 pb-20 sm:px-8">
        <main id="main-content" className="void-md mx-auto max-w-2xl" tabIndex={-1}>
          <img
            src="https://github.com/erkamyaman.png?size=192"
            width={96}
            height={96}
            alt="Erkam Yaman"
            className="mb-6 rounded-full"
          />
          <h1>About the author</h1>
          <p>
            I'm Erkam Yaman (kæm), a software engineer in Istanbul. Most days I work inside the Angular codebase, mainly
            Angular's docs and docs-infra, and I created <a href="https://angularians.dev">Angularians</a>,{' '}
            <a href="https://slide-ng.dev">slide.ng</a> and <a href="https://angular-docs.tr">angular-docs.tr</a>. I work
            at Kolay Randevu.
          </p>
          <p>
            I built Rust Anything because moving an app to Rust is a long, complex process, and I wanted to automate
            it so developers all around the world can ship apps that use less storage and run faster. It doesn't
            translate code. It records what the original app does and
            checks every piece of the Rust rewrite against that recording before it moves on.
          </p>
          <p>
            Two real ports so far, both <a href={REAL_WORLD}>measured and written up</a>: Prettier, 18.8x to 19.8x faster
            as a CLI on 2,360 files with byte-identical output, and json-server, whose idle memory went from 74 MB to 3.5
            MB while passing 514 of 514 recorded HTTP cases.
          </p>
          <p>
            Rust Anything is free and open source (MIT OR Apache-2.0). You can reach me at{' '}
            <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a> or on GitHub as{' '}
            <a href="https://github.com/erkamyaman">@erkamyaman</a>. More about me and my other projects is on{' '}
            <a href="https://erkamyaman.dev">erkamyaman.dev</a>.
          </p>
        </main>
      </div>
      <Footer locale={LOCALE} />
    </div>
  )
}
