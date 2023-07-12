<script>
	import Header from './Header.svelte';
	import './styles.css';

	// const handleGoogleCallback = ({ credential }) => {
	// 	debugger
	// }

	// google.accounts.id.initialize({
	// 	client_id: '181396477895-mif6hcekhvhi32up28g49hve07vlvchm.apps.googleusercontent.com',
	// 	callback: handleGoogleCallback
	// });
	// google.accounts.id.prompt();
</script>

<svelte:head>
	<script src="https://accounts.google.com/gsi/client" async></script>
</svelte:head>

<div class="app">
	<Header />

	<main>
		<slot />
	</main>

	<footer>
		<p>visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to learn SvelteKit</p>
	</footer>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		width: 100%;
		max-width: 64rem;
		margin: 0 auto;
		box-sizing: border-box;
	}

	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 12px;
	}

	footer a {
		font-weight: bold;
	}

	@media (min-width: 480px) {
		footer {
			padding: 12px 0;
		}
	}
</style>
