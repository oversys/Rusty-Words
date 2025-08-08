<script>
</script>

<template>
	<div class="header-container">
		<h2>
			{{
				$route.path === '/' 
					? 'My Words' 
					: $route.path === '/add' 
					? 'Add Word' 
					: $route.path === '/word' 
					? 'Word Details' 
					: ''
			}}
		</h2>
	</div>
</template>

<style scoped>
.header-container {
	position: fixed;
	top: 0;
	width: 100%;
	display: flex;
	justify-content: space-around;
	padding: 1rem 0.5rem;
	background-color: #FFFFFC;
	border-bottom: 1.75px solid #D4CDC3;
}
</style>

