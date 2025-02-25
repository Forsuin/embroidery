<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import Svelecte from "svelecte";

	let options = [
		"First",
		"Second",
		"Third",
		"Fourth",
		"Fifth",
		"Sixth",
		"Seventh",
		"Eight",
		"Ninth",
		"Tenth",
	];
</script>

<Sidebar.Root
	collapsible="none"
	class="top-(--header-height) h-[calc(100svh-var(--header-height))]! min-w-full"
>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Search</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					<Svelecte
						searchable
						clearable
						multiple
						allowEditing
						keepCreated
						placeholder="Search Tags"
						{options}
					/>
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
		<Sidebar.Separator />
		<Sidebar.Group>
			<Sidebar.GroupLabel>Tags</Sidebar.GroupLabel>
			<Sidebar.GroupContent class="flex flex-wrap">
				<div class="space-y-4">
					<div class="flex gap-4">
						<button
							type="button"
							class="chip preset-filled-primary-500">Chip</button
						>
						<button type="button" class="chip preset-tonal-primary"
							>Chip</button
						>
						<button
							type="button"
							class="chip preset-outlined-primary-500"
							>Chip</button
						>
					</div>
					<div class="flex gap-4">
						<button
							type="button"
							class="chip preset-filled-secondary-500"
							>Chip</button
						>
						<button
							type="button"
							class="chip preset-tonal-secondary">Chip</button
						>
						<button
							type="button"
							class="chip preset-outlined-secondary-500"
							>Chip</button
						>
					</div>
					<div class="flex gap-4">
						<button
							type="button"
							class="chip preset-filled-tertiary-500">Chip</button
						>
						<button type="button" class="chip preset-tonal-tertiary"
							>Chip</button
						>
						<button
							type="button"
							class="chip preset-outlined-tertiary-500"
							>Chip</button
						>
					</div>
					<div class="flex gap-4">
						<button
							type="button"
							class="chip preset-filled-success-500">Chip</button
						>
						<button type="button" class="chip preset-tonal-success"
							>Chip</button
						>
						<button
							type="button"
							class="chip preset-outlined-success-500"
							>Chip</button
						>
					</div>
					<div class="flex gap-4">
						<button
							type="button"
							class="chip preset-filled-warning-500">Chip</button
						>
						<button type="button" class="chip preset-tonal-warning"
							>Chip</button
						>
						<button
							type="button"
							class="chip preset-outlined-warning-500"
							>Chip</button
						>
					</div>
					<div class="flex gap-4">
						<button
							type="button"
							class="chip preset-filled-error-500">Chip</button
						>
						<button type="button" class="chip preset-tonal-error"
							>Chip</button
						>
						<button
							type="button"
							class="chip preset-outlined-error-500">Chip</button
						>
					</div>
					<div class="flex gap-4">
						<button
							type="button"
							class="chip preset-filled-surface-500">Chip</button
						>
						<button type="button" class="chip preset-tonal-surface"
							>Chip</button
						>
						<button
							type="button"
							class="chip preset-outlined-surface-500"
							>Chip</button
						>
					</div>
				</div>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
</Sidebar.Root>
