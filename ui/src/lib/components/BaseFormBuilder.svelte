<script lang="ts">
	import {
		type BuilderAPI,
		FormBuilder,
		type RenderAPI
	} from '@pragmatic-engineering/svelte-form-builder-community';
	import type {
		FormDefinition,
		views
	} from '@pragmatic-engineering/svelte-form-builder-community/Utils/types';

	export let builderApi: typeof BuilderAPI | undefined = undefined;
	export let renderApi: typeof RenderAPI | undefined = undefined;
	export let formDefinition: FormDefinition[] = [];
	export let view: views = 'build';
</script>

<div class="form-builder-wrapper w-full">
	<FormBuilder
		bind:builderAPI={builderApi}
		bind:renderAPI={renderApi}
		options={{
			allowHtmlLabels: false,
			editOnAdd: true,
			enableTabs: false,
			disabledViews: {
				settings: true,
				conditions: true,
				tools: true
			},
			disableComponents: [
				'Audio',
				'Button',
				'Canvas',
				'Color',
				'Week',
				'Stars',
				'Password',
				'Month',
				'Meter',
				'Hidden',
				'Progress',
				'Time',
				'DateTime',
				'Video'
			],
			view,
			formDefinition
		}}
		{...$$props}
	/>
</div>

<style>
	.form-builder-wrapper :global(.svelte-fb-root textarea) {
		border-radius: 0.5rem;
	}

	.form-builder-wrapper :global(.svelte-fb-root input) {
		border-radius: 0.5rem;
	}

	.form-builder-wrapper :global(.svelte-fb-root label) {
		--tw-text-opacity: 1;
		color: rgb(156 163 175 / var(--tw-text-opacity));
	}

	.form-builder-wrapper :global(.svelte-fb-root .header) {
		--tw-text-opacity: 1;
		color: rgb(156 163 175 / var(--tw-text-opacity));
		background-color: transparent !important;
		padding-bottom: 2rem;
	}
</style>
