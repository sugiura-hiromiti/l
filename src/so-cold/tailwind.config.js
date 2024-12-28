/** @type {import('tailwindcss').Config} */
module.exports = {
	mode: "all",
	content: ["./{src,examples}/**/*.{rs,html,css}", "./target/**/*.html"],
	theme: {
		//デフォルトのfont-sansクラスのフォントを乗っ取りたい時はこっち！
		extend: {
			fontFamily: {
				'sans': ['snf']
			}
		},
		// fontFamily: {
		// 	'nerd'の部分はなんでも良いです 自分の好きな名前にしましょう
		// 	'nerd': ['snf']
		// }
	},
	plugins: [],
};
