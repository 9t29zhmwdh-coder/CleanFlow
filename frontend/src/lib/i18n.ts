import { create } from 'zustand'

export type Lang = 'en' | 'de'

const STORAGE_KEY = 'cleanflow_lang'

let currentLang: Lang = (localStorage.getItem(STORAGE_KEY) as Lang) || 'en'

export function getLang(): Lang {
  return currentLang
}

interface LangState {
  lang: Lang
  setLang: (l: Lang) => void
  toggle: () => void
}

export const useLangStore = create<LangState>((set) => ({
  lang: currentLang,
  setLang: (l) => {
    currentLang = l
    localStorage.setItem(STORAGE_KEY, l)
    set({ lang: l })
  },
  toggle: () => {
    const next: Lang = currentLang === 'en' ? 'de' : 'en'
    currentLang = next
    localStorage.setItem(STORAGE_KEY, next)
    set({ lang: next })
  },
}))

const translations = {
  en: {
    tagline: 'AI-powered file organizer',
    navHome: 'Home', navScan: 'Scan', navPlan: 'Plan', navSettings: 'Settings',

    cardScanTitle: 'Scan Directory', cardScanDesc: 'Analyse a folder and get an AI-powered organisation plan',
    cardSettingsTitle: 'Settings', cardSettingsDesc: 'Configure AI provider, rules and output preferences',

    scanTitle: 'Scan Directory', pathPlaceholder: '~/Downloads', browse: 'Browse', scan: 'Scan',
    statFiles: 'Files', statJunk: 'Junk', statDuplicates: 'Duplicates', statTotalSize: 'Total Size',
    generatePlan: 'Generate CleanFlow Plan',

    planTitle: 'CleanFlow Plan', newScan: 'New Scan',
    statActions: 'Actions', statFreed: 'Freed', statAi: 'AI',
    cleanflowExecuteAll: 'CleanFlow: Execute All',
    selectedCount: '{{n}} / {{total}} selected', executeSelected: 'Execute selected',

    reasonRule: 'Rule: {{name}}', reasonAi: 'AI: {{explanation}}',
    reasonDuplicate: 'Duplicate', reasonJunk: 'Junk: {{type}}', reasonOldVersion: 'Old version',
    tagPrefix: 'Tag:', createPrefix: 'Create:',

    justNow: 'just now', minutesAgo: '{{n}}m ago', hoursAgo: '{{n}}h ago', daysAgo: '{{n}}d ago',

    settingsTitle: 'Settings', settingsDesc: 'AI provider, API keys, custom rules.',
  },
  de: {
    tagline: 'KI-gestützter Datei-Organizer',
    navHome: 'Start', navScan: 'Scan', navPlan: 'Plan', navSettings: 'Einstellungen',

    cardScanTitle: 'Ordner scannen', cardScanDesc: 'Analysiere einen Ordner und erhalte einen KI-gestützten Organisationsplan',
    cardSettingsTitle: 'Einstellungen', cardSettingsDesc: 'KI-Anbieter, Regeln und Ausgabe-Einstellungen konfigurieren',

    scanTitle: 'Ordner scannen', pathPlaceholder: '~/Downloads', browse: 'Durchsuchen', scan: 'Scannen',
    statFiles: 'Dateien', statJunk: 'Müll', statDuplicates: 'Duplikate', statTotalSize: 'Gesamtgrösse',
    generatePlan: 'CleanFlow-Plan erstellen',

    planTitle: 'CleanFlow-Plan', newScan: 'Neuer Scan',
    statActions: 'Aktionen', statFreed: 'Freigegeben', statAi: 'KI',
    cleanflowExecuteAll: 'CleanFlow: Alles ausführen',
    selectedCount: '{{n}} / {{total}} ausgewählt', executeSelected: 'Auswahl ausführen',

    reasonRule: 'Regel: {{name}}', reasonAi: 'KI: {{explanation}}',
    reasonDuplicate: 'Duplikat', reasonJunk: 'Müll: {{type}}', reasonOldVersion: 'Alte Version',
    tagPrefix: 'Tag:', createPrefix: 'Erstellen:',

    justNow: 'gerade eben', minutesAgo: 'vor {{n}} Min.', hoursAgo: 'vor {{n}} Std.', daysAgo: 'vor {{n}} Tagen',

    settingsTitle: 'Einstellungen', settingsDesc: 'KI-Anbieter, API-Schlüssel, eigene Regeln.',
  },
} as const

type TranslationKey = keyof typeof translations.en

function interpolate(str: string, vars?: Record<string, string | number>): string {
  if (!vars) return str
  return str.replace(/\{\{(\w+)\}\}/g, (_, key) => String(vars[key] ?? ''))
}

export function t(key: TranslationKey, vars?: Record<string, string | number>): string {
  const str = translations[currentLang][key] ?? key
  return interpolate(str, vars)
}

export function useT() {
  const lang = useLangStore((s) => s.lang)
  return (key: TranslationKey, vars?: Record<string, string | number>) => {
    const str = translations[lang][key] ?? key
    return interpolate(str, vars)
  }
}
