import Navbar from '../../components/site/Navbar' with { island: 'load' }
import Hero from '../../components/landing/Hero' with { island: 'load' }
import InstallBox from '../../components/landing/InstallBox' with { island: 'visible' }
import Home from '../../components/landing/Home'
import Footer from '../../components/site/Footer'
import { messages } from '../../lib/i18n'
import { localizedNav } from '../../lib/nav'

const LOCALE = 'es'

export default function Index() {
  const m = messages(LOCALE)
  return (
    <div className="bg-bg text-fg">
      <header className="sticky top-0 z-50 border-b border-line bg-bg/85 backdrop-blur-md">
        <Navbar currentPath="/es" locale={LOCALE} ui={m.ui} nav={localizedNav(LOCALE)} />
      </header>
      <main id="main-content">
        <Home
          locale={LOCALE}
          hero={<Hero t={m.hero} locale={LOCALE} />}
          install={<InstallBox ui={m.ui} agent={[m.home.agentPrompt, m.home.agentRestart]} />}
        />
      </main>
      <Footer locale={LOCALE} />
    </div>
  )
}
