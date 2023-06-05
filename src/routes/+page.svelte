<script lang="ts">
	import { getClient, ResponseType, Response } from '@tauri-apps/api/http';
	import { getMatches } from '@tauri-apps/api/cli';
	import { open } from '@tauri-apps/api/dialog';
	import { appWindow } from '@tauri-apps/api/window';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	let selectOptions: string[] = [];
	let folder: string | undefined = undefined;
	let text1: string = '';
	let text2: string = '';
	let checked = false;
	onMount(async () => {
		const client = await getClient();

		try {
			let resp: Response<string[]> = await client.get(
				'http://192.168.1.217:5002/designers/projectsnames',
				{
					timeout: 30,
					// the expected response type
					responseType: ResponseType.JSON
				}
			);
			if (resp.ok) {
				selectOptions = resp.data;
			}
		} catch (e) {
			console.error(e);
		}
		let { args } = await getMatches();
		try {
			if (args['folder'].occurrences > 0) {
				folder = args['folder'].value?.toString();
			}
			if (folder === undefined || folder.length == 0) {
				// Open a selection dialog for directories
				const selected = await open({
					directory: true,
					multiple: false
				});
				console.log(selected);
				if (selected === null) {
					await appWindow.close();
				} else if (Array.isArray(selected)) {
					folder = trimChar(selected[1], '"');
				} else {
					folder = trimChar(selected, '"');
				}
			}
		} catch (e) {
			console.log(e);
		}
	});

	function trimChar(string: string, charToRemove: string): string {
		if (string.charAt(0) === charToRemove) {
			string = string.substring(1);
		}

		if (string.charAt(string.length - 1) == charToRemove) {
			string = string.substring(0, string.length - 1);
		}

		return string;
	}

	async function Submit() {
		try {
			await invoke('change_names', {
				comment: text1,
				errand: text2,
				folder: folder,
				addNumbers: checked
			});
			await appWindow.close();
		} catch (err) {
			console.error(err);
		}
	}
</script>

<form>
	<!-- Grid -->
	<div class="grid">
		<!-- Markup example 1: input is inside label -->
		<label for="text1">
			Wpisz komentarz/numer zamówienia
			<input type="text" name="text1" placeholder="..." bind:value={text1} />
		</label>

		<label for="text2">
			Lub wybierz z listy zamówień
			<!-- Select -->
			<select bind:value={text2}>
				<option value="" selected>Wybierz</option>
				{#each selectOptions as optionValue}
					<option value={optionValue}>{optionValue}</option>
				{/each}
			</select>
		</label>
		<label for="checked">
			<input type="checkbox" id="checked" name="checked" bind:checked />
			Dodaj numerację ścieżek
		</label>
	</div>
	<!-- Button -->
	<button type="submit" on:click={Submit}>Zamień nazwy</button>
</form>
