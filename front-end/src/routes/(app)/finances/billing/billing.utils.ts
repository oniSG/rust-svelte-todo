import { BillingCondition } from '$lib/api/generated/rustSvelteTodo.schemas';

export const CONDITION_LABELS: Record<string, string> = {
	[BillingCondition.less_than]: 'Less than',
	[BillingCondition.more_than]: 'More than'
};

export function conditionVariant(c: string): 'secondary' | 'default' {
	return c === BillingCondition.less_than ? 'secondary' : 'default';
}

export function fmtPrice(n: number | null | undefined): string {
	if (n == null) return '—';
	return n.toLocaleString('en-US');
}

export function parsePrice(v: string | number): number | null {
	const s = String(v).trim();
	return s === '' ? null : Number(s);
}
