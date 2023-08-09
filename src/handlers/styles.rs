pub const RESET: &'static str = r#"*,::after,::before{box-sizing:border-box;border-width:0;border-style:solid;border-color:#e5e7eb}html{line-height:1.5;-webkit-text-size-adjust:100%;-moz-tab-size:4;tab-size:4;font-family:ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,"Helvetica Neue",Arial,"Noto Sans",sans-serif,"Apple Color Emoji","Segoe UI Emoji","Segoe UI Symbol","Noto Color Emoji"}body{margin:0;line-height:inherit}hr{height:0;color:inherit;border-top-width:1px}abbr:where([title]){text-decoration:underline dotted}h1,h2,h3,h4,h5,h6{font-size:inherit;font-weight:inherit}a{color:inherit;text-decoration:inherit}b,strong{font-weight:bolder}code,kbd,pre,samp{font-family:ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,"Liberation Mono","Courier New",monospace;font-size:1em}small{font-size:80%}sub,sup{font-size:75%;line-height:0;position:relative;vertical-align:baseline}sub{bottom:-.25em}sup{top:-.5em}table{text-indent:0;border-color:inherit;border-collapse:collapse}button,input,optgroup,select,textarea{font-family:inherit;font-feature-settings:inherit;font-variation-settings:inherit;font-size:100%;font-weight:inherit;line-height:inherit;color:inherit;margin:0;padding:0}button,select{text-transform:none}[type=button],[type=reset],[type=submit],button{-webkit-appearance:button;background-color:transparent;background-image:none}:-moz-focusring{outline:auto}:-moz-ui-invalid{box-shadow:none}progress{vertical-align:baseline}::-webkit-inner-spin-button,::-webkit-outer-spin-button{height:auto}[type=search]{-webkit-appearance:textfield;outline-offset:-2px}::-webkit-search-decoration{-webkit-appearance:none}::-webkit-file-upload-button{-webkit-appearance:button;font:inherit}summary{display:list-item}blockquote,dd,dl,figure,h1,h2,h3,h4,h5,h6,hr,p,pre{margin:0}fieldset{margin:0;padding:0}legend{padding:0}menu,ol,ul{list-style:none;margin:0;padding:0}textarea{resize:vertical}input::placeholder,textarea::placeholder{opacity:1;color:#9ca3af}[role=button],button{cursor:pointer}:disabled{cursor:default}audio,canvas,embed,iframe,img,object,svg,video{display:block;vertical-align:middle}img,video{max-width:100%;height:auto}[hidden]{display:none}"#;

pub const SYSTEM: &'static str = r#"
:root {
	--font-emoji: "Apple Color Emoji","Segoe UI Emoji","Segoe UI Symbol","Noto Color Emoji";
	--font-text: "neue-haas-grotesk-text", "SF Pro Text", system-ui, sans-serif, var(--font-emoji);
	--font-display: "neue-haas-grotesk-display", "SF Pro Display", system-ui, sans-serif, var(--font-emoji);
	--row-height: 24px;
	--max-width: calc(var(--row-height) * 28);

	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;

	scroll-behavior: smooth;

	font-size: 18px;
	line-height: var(--row-height);
	font-family: var(--font-text);
}
body {
	padding-top: var(--row-height);
	padding-bottom: var(--row-height);
	padding-left: calc(var(--row-height) / 2);
	padding-right: calc(var(--row-height) / 2);

	max-width: var(--max-width);
	margin-left: auto;
	margin-right: auto;

	display: flex;
	flex-direction: column;
	gap: calc(var(--row-height) * 2);
}
main {
	display: flex;
	flex-direction: column;
	gap: calc(var(--row-height) * 2);
}
h1 {
	font-size: 42px;
	line-height: calc(var(--row-height) * 2);
	font-family: var(--font-display);
	font-weight: 600;
	font-style: normal;
	text-wrap: balance;
	border-top: 2px solid #000;
}
h2 {
	font-weight: 700;
}
.btn1 {
	display: flex;
	height: calc(var(--row-height) * 2);
	justify-content: center;
	align-items: center;
	align-self: stretch;
	background: #000;
	width: 100%;
}
.btn1 > span {
	color: #FFF;
	text-align: center;
	font-family: var(--font-text);
	font-size: 18px;
	font-style: normal;
	font-weight: 700;
	line-height: var(--row-height);
}
@media (min-width: 528px) {
	.btn1 {
		grid-column: 2 / span 2;
	}
}
dl {
	display: grid;
	grid-template-columns: 1fr 1fr 1fr 1fr;
	column-gap: calc(var(--row-height) / 2);
	row-gap: var(--row-height);
}
dl > dt {
	font-style: italic;
	grid-column: 1 / span 3;
}
dl > dd {
	grid-column: 2 / span 3;
}
ol > li {
	display: grid;
	grid-template-columns: 1fr 1fr 1fr 1fr;
	column-gap: calc(var(--row-height) / 2);
	row-gap: var(--row-height);
}
li {
	counter-increment: my-counter;
}
li::before {
	content: counter(my-counter) ". ";
	grid-column: 1;
}
section {
	display: grid;
	grid-template-columns: 1fr 1fr 1fr 1fr;
	column-gap: calc(var(--row-height) / 2);
	row-gap: var(--row-height);
}
section * {
	grid-column: 1 / span 4;
}
@media (min-width: 528px) {
	section {
		> h1, > h2, > h3, > h4, > h5, > h6 {
			grid-column: 1 / span 3;
		}
		> p {
			grid-column: 2 / span 3;
		}
	}
}
ol {
	display: flex;
	flex-direction: column;
	gap: var(--row-height);
}
ol > li > p {
	grid-column: 2 / span 3;
}
label {
	text-transform: capitalize;
	font-style: italic;
	grid-column: 1 / span 4;
}
input {
	height: calc(var(--row-height) * 1.5);
	border: 1px solid #000;
	grid-column: 1 / span 4;
	padding-left: calc(var(--row-height) / 4);
}
label + input {
	margin-top: calc(var(--row-height) / -2);
}
small {
	padding-top: 10px;
	line-height: 14px;
	margin-top: calc(var(--row-height) * -1);
	font-size: 14px;
	grid-column: 1 / span 3;
}
header {
	display: none;
}
@media (min-width: 840px) {
	header {
		display: block;
		writing-mode: vertical-rl;
		transform: rotate(180deg);
		position: fixed;
		bottom: calc(var(--row-height) * 2);
		left: var(--row-height);
	}
}
#hero img {
	height: calc(var(--row-height) * 8);
	width: 100%;
	object-fit: cover;
	object-position: center;
}
#noscript { 
	position: fixed;
	top: 0;
	left: 0;
	background-color: #FFF;
	z-index: 999;
	height: 100%;
	width: 100%;
	padding-top: var(--row-height);
	padding-left: calc(var(--row-height) / 2);
	padding-right: calc(var(--row-height) / 2);
}
#entry {
	display: grid;
	grid-template-columns: 1fr 1fr 1fr 1fr;
	column-gap: calc(var(--row-height) / 2);
	row-gap: var(--row-height);
}
#entry.valid {
	> input {
		border-color: #4E884A;
	}
	> small {
		color: #4E884A;
	}
}
#entry.invalid {
	> input {
		border-color: #C3523F;
	}
	> small {
		color: #C3523F;
	}
}
"#;
