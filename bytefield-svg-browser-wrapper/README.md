


### bytefield-svg browser wrapper

[bytefield-svg browser wrapper](https://github.com/LMinJae/bytefield-svg-browser-wrapper.git)

### use

[bytefield-svg](https://github.com/Deep-Symmetry/bytefield-svg) browser wrapper

Use case: Hugo translated code block(code element with classname and dataset) client-side rendering
```html
<script src="https://cdn.jsdelivr.net/gh/LMinJae/bytefield-svg-browser-wrapper/dist/bundle.js"></script>
<script>
	document.addEventListener('DOMContentLoaded', () => {
		document.body.querySelectorAll('code.language-bytefield').forEach(n => {
			n.outerHTML = bytefield.generate(n.innerText, { 'embedded': true });
		});
	});
</script>

<pre tabindex="0"><code class="language-bytefield" data-lang="bytefield">(def row-header-fn {})
	(draw-column-headers)
	(draw-box "Address" {:span 4})
	(draw-box "Size" {:span 2})
	(draw-box 0 {:span 2})
	(draw-gap "Payload")
	(draw-bottom)
</code></pre>
```

### build

```sh
npm install @rollup/plugin-node-resolve --save-dev
npx rollup -c --bundleConfigAsCjs
```
