import { createRouter, createWebHistory } from 'vue-router'
import WordList from '../views/WordList.vue'
import AddWord from '../views/AddWord.vue'
import WordDetails from '../views/WordDetails.vue'

const router = createRouter({
	history: createWebHistory(),
	routes: [
		{
			path: '/',
			component: WordList,
			meta: {
				title: 'Rusty Words',
			},
		},
		{
			path: '/add',
			component: AddWord,
			meta: {
				title: 'Add Word | Rusty Words',
			},
		},
		{
			path: '/word',
			component: WordDetails,
			meta: {
				title: 'Word Details | Rusty Words',
			},
		},
		{
			path: '/:pathMatch(.*)*',
			redirect: '/',
		},
	],
	scrollBehavior(to, from, savedPosition) {
		return { top: 0 }
	},
})

router.beforeEach((to, from) => {
	document.title = to.meta.title || 'Rusty Words';
})

export default router
