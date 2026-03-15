import { useSettings } from './useSettings'

export function usePrivacy() {
  const settings = useSettings()

  return {
    privacy: settings.privacy,
    togglePrivacy: settings.togglePrivacy,
    setPrivacy: settings.setPrivacy
  }
}
