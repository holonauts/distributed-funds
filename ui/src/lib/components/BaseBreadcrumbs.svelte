<script lang="ts">
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
	import { page } from '$app/stores';
	import { generateBreadcrumbs } from '$lib/utils/breadcrumbs';

	export let title: string;
	export let replacements: { [name: string]: string } = {};
	export let disabled: string[] = [];

	$: breadcrumbs = generateBreadcrumbs($page.url.pathname, replacements);
</script>

<div class="mb-14 flex justify-start">
	<Breadcrumb
		olClass="inline-flex items-center rtl:space-x-reverse"
		aria-label="Default breadcrumb example"
	>
		<BreadcrumbItem href="/" home></BreadcrumbItem>
		{#each breadcrumbs as crumb}
			<BreadcrumbItem
				href={disabled.includes(crumb.label) ? undefined : crumb.href}
				linkClass="mr-0 text-sm font-medium text-gray-700 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white capitalize"
				>{crumb.label}</BreadcrumbItem
			>
		{/each}
		<BreadcrumbItem spanClass="text-sm font-medium text-gray-700 dark:text-gray-300 capitalize">
			{title}
		</BreadcrumbItem>
	</Breadcrumb>
</div>
