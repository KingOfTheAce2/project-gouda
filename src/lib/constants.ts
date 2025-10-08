// Type related constants
export const PROVIDER_OLLAMA = 'Ollama';
export const PROVIDER_CUSTOM = 'CUSTOM';
export const PROVIDER_UNKNOWN = 'Unknown';
export const SUPPORTED_PROVIDERS = [
  PROVIDER_OLLAMA,
] as const;
export const ALL_PROVIDERS = [
  ...SUPPORTED_PROVIDERS,
  PROVIDER_UNKNOWN,
] as const;
export const PROVIDER_STYLES = {
  [PROVIDER_OLLAMA]: {
    icon: {
      light: 'ollama_logo_b.svg',
      dark: 'ollama_logo_w.svg',
    },
    color: {
      light: '#EEEEEE',
      dark: '#999999',
    },
  },
  [PROVIDER_CUSTOM]: {
    icon: {
      light: 'unknown_logo_b.svg',
      dark: 'unknown_logo_w.svg',
    },
    color: {
      light: '#CCCCCC',
      dark: '#666666',
    },
  },
  [PROVIDER_UNKNOWN]: {
    icon: {
      light: 'unknown_logo_b.svg',
      dark: 'unknown_logo_w.svg',
    },
    color: {
      light: '#CCCCCC',
      dark: '#CCCCCC',
    },
  },
};

// Message types
export const MESSAGE_USER = 0;
export const MESSAGE_BOT = 1;
export const MESSAGE_SYSTEM = 2;

// Message content item type
export const CONTENT_ITEM_TYPE_TEXT = 0;
export const CONTENT_ITEM_TYPE_IMAGE = 1;
export const CONTENT_ITEM_TYPES = [
  CONTENT_ITEM_TYPE_TEXT,
  CONTENT_ITEM_TYPE_IMAGE,
] as const;

// Stream keywords
export const STREAM_START = '[[START]]';
export const STREAM_DONE = '[[DONE]]';
export const STREAM_ERROR = '[[ERROR]]';
export const STREAM_STOPPED = '[[STOPPED]]';

// Setting keys
export const SETTING_USER_DEFAULT_MODEL = 'user:default_model';
export const SETTING_USER_ENTER_TO_SEND = 'user:enter_to_send';
export const SETTING_DISPLAY_LANGUAGE = 'display:language';
export const SETTING_DISPLAY_THEME = 'display:darkmode';
export const SETTING_IS_SIDEBAR_PINNED = 'display:is_sidebar_pinned';
export const SETTING_IS_WIDE_SCREEN = 'display:is_wide_screen';
export const SETTING_PROFILE_NAME = 'profile:name';
export const SETTING_MODELS_CONTEXT_LENGTH = 'models:context_length';
export const SETTING_MODELS_MAX_TOKENS = 'models:max_tokens';
export const SETTING_NETWORK_PROXY = 'network:proxy';

// Defaults
export const DEFAULT_DATE_FORMAT = 'MMM D, YYYY';
export const DEFAULT_DATETIME_FORMAT = 'HH:mm MMM D, YYYY';
export const DEFAULT_PROFILE_NAME = 'ME';
export const DEFAULT_CONTEXT_LENGTH = 1;
export const DEFAULT_MAX_TOKENS = 256;
export const DEFAULT_IS_SIDEBAR_PINNED = false;

// Constatns
export const MAX_NUM_OF_UPLOAD_FILES = 10;
