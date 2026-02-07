import { derived } from 'svelte/store';
import { locale } from './stores';

export const translations = {
	en: {
		app: {
			title: 'DNS Switcher',
			subtitle: 'Easily change your DNS settings'
		},
		adapter: {
			label: 'Network Adapter',
			select: 'Select Network Adapter'
		},
		dns: {
			current: 'Current DNS',
			primary: 'Primary DNS',
			secondary: 'Secondary DNS'
		},
		status: {
			dhcp: 'DHCP (Automatic)',
			notSet: 'Not Set',
			unknown: 'Unknown',
			loading: 'Loading...',
			connected: 'Connected',
			disconnected: 'Disconnected',
			reachable: 'Reachable',
			unreachable: 'Unreachable'
		},
		tabs: {
			presets: 'DNS Presets',
			custom: 'Custom DNS'
		},
		custom: {
			title: 'Custom DNS Settings',
			description: 'Enter your custom DNS server addresses',
			examples: 'Common DNS Examples'
		},
		actions: {
			apply: 'Apply DNS',
			applying: 'Applying...',
			reset: 'Reset to DHCP',
			test: 'Test DNS',
			flush: 'Flush DNS Cache',
			testPrimary: 'Test Primary',
			testSecondary: 'Test Secondary'
		},
		success: {
			dnsApplied: 'DNS settings for {name} applied successfully',
			customDnsApplied: 'Custom DNS settings applied successfully',
			dnsReset: 'DNS reset to DHCP successfully',
			cacheFlushed: 'DNS cache flushed successfully'
		},
		errors: {
			loadFailed: 'Failed to load initial data',
			dnsFetchFailed: 'Failed to fetch DNS settings',
			noAdapter: 'Please select a network adapter',
			primaryRequired: 'Primary DNS address is required',
			dnsApplyFailed: 'Failed to apply DNS settings',
			dnsResetFailed: 'Failed to reset DNS',
			invalidIp: 'Invalid IP address format'
		},
		preset: {
			category: {
				all: 'All',
				public: 'Public',
				security: 'Security',
				privacy: 'Privacy',
				family: 'Family Safe',
				adblock: 'Ad Blocking',
				gaming: 'Gaming'
			}
		},
		settings: {
			title: 'Settings',
			theme: 'Theme',
			language: 'Language',
			light: 'Light',
			dark: 'Dark'
		},
		about: {
			title: 'About',
			version: 'Version'
		},
		testing: 'Testing...',
		responseTime: 'Response Time: {time}ms'
	},
	ru: {
		app: {
			title: 'DNS Переключатель',
			subtitle: 'Легко меняйте настройки DNS'
		},
		adapter: {
			label: 'Сетевой адаптер',
			select: 'Выберите сетевой адаптер'
		},
		dns: {
			current: 'Текущий DNS',
			primary: 'Основной DNS',
			secondary: 'Дополнительный DNS'
		},
		status: {
			dhcp: 'DHCP (Автоматически)',
			notSet: 'Не установлено',
			unknown: 'Неизвестно',
			loading: 'Загрузка...',
			connected: 'Подключено',
			disconnected: 'Отключено',
			reachable: 'Доступен',
			unreachable: 'Недоступен'
		},
		tabs: {
			presets: 'Готовые DNS',
			custom: 'Свой DNS'
		},
		custom: {
			title: 'Настройки DNS',
			description: 'Введите адреса своих DNS-серверов',
			examples: 'Популярные DNS примеры'
		},
		actions: {
			apply: 'Применить DNS',
			applying: 'Применение...',
			reset: 'Сбросить на DHCP',
			test: 'Тест DNS',
			flush: 'Очистить кэш DNS',
			testPrimary: 'Тест основного',
			testSecondary: 'Тест дополнительного'
		},
		success: {
			dnsApplied: 'DNS настройки для {name} успешно применены',
			customDnsApplied: 'Пользовательские DNS настройки успешно применены',
			dnsReset: 'DNS успешно сброшен на DHCP',
			cacheFlushed: 'Кэш DNS успешно очищен'
		},
		errors: {
			loadFailed: 'Не удалось загрузить начальные данные',
			dnsFetchFailed: 'Не удалось получить настройки DNS',
			noAdapter: 'Пожалуйста, выберите сетевой адаптер',
			primaryRequired: 'Требуется адрес основного DNS',
			dnsApplyFailed: 'Не удалось применить настройки DNS',
			dnsResetFailed: 'Не удалось сбросить DNS',
			invalidIp: 'Неверный формат IP-адреса'
		},
		preset: {
			category: {
				all: 'Все',
				public: 'Публичные',
				security: 'Безопасность',
				privacy: 'Конфиденциальность',
				family: 'Семейные',
				adblock: 'Блокировка рекламы',
				gaming: 'Для игр'
			}
		},
		settings: {
			title: 'Настройки',
			theme: 'Тема',
			language: 'Язык',
			light: 'Светлая',
			dark: 'Тёмная'
		},
		about: {
			title: 'О программе',
			version: 'Версия'
		},
		testing: 'Тестирование...',
		responseTime: 'Время отклика: {time}мс'
	}
};


function getNestedValue(obj: any, path: string): string {
	const keys = path.split('.');
	let value = obj;
	
	for (const key of keys) {
		value = value?.[key];
		if (value === undefined) break;
	}
	
	return value;
}

function interpolate(text: string, params?: Record<string, any>): string {
	if (!params) return text;
	
	return text.replace(/\{(\w+)\}/g, (match, key) => {
		return params[key] !== undefined ? String(params[key]) : match;
	});
}

// Create a derived store for translation function
export const t = derived(locale, ($locale) => {
	return (key: string, params?: Record<string, any>): string => {
		const lang = translations[$locale as keyof typeof translations] || translations.en;
		const value = getNestedValue(lang, key);
		
		if (value === undefined) {
			console.warn(`Translation missing for key: ${key}`);
			return key;
		}
		
		return interpolate(value, params);
	};
});