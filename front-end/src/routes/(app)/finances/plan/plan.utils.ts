export function fmtDate(dateStr: string): string {
	return new Date(dateStr).toLocaleDateString('en-US', { month: 'short', year: 'numeric' });
}

export function fmtCurrency(n: number): string {
	return n.toLocaleString('en-US');
}
