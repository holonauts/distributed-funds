<script lang="ts">
	import { Navbar, NavBrand, NavUl, NavLi, DarkMode } from 'flowbite-svelte';
	import { page } from '$app/stores';

	$: activeUrl = $page.url.pathname;
</script>

<div class="relative flex items-start justify-center">
	<Navbar let:hidden>
		<NavBrand href="/">
			<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
				Public Grant Manager
			</span>
		</NavBrand>
		<NavUl {hidden} {activeUrl} class="flex items-center">
			<NavLi href="/grant-pools">Grant Pools</NavLi>
			<NavLi href="/applications/mine">My Applications</NavLi>
			<NavLi href="/evaluations/mine">My Evaluations</NavLi>
			<DarkMode
				btnClass="block py-2 pe-4 ps-3 md:p-0 rounded text-gray-700 hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-primary-700 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
			/>
		</NavUl>
	</Navbar>
</div>
